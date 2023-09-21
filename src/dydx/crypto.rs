use primitive_types::U512;
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};

use crate::dydx::constants::*;
use crate::dydx::ec_math::ECMath;
use super::{Position, Side, Markets};


// Calculates the pedersen hash for the given order.
pub fn order_hash(client_id: String, position: &Position, market: Markets, fee: String, expiration_seconds: u64, position_id: u64, testnet: bool) -> U512 {

    let mut buying = true;
    if position.side == Side::SELL { buying = false; }

    let nonce = convert_nonce(client_id);
    let expiration_hours = (((expiration_seconds as f64)/ONE_HOUR_IN_SECONDS as f64).ceil() as u64 + SIGNATURE_EXPIRATION_TIME) as u128;

    let asset_id_sell;
    let asset_id_buy;
    let qa_sell;
    let qa_buy;
    let qa_fee;
    if buying {
        if testnet { asset_id_sell = *USD_ID_TESTNET; }
        else { asset_id_sell = *USD_ID_MAINNET; }
        asset_id_buy = market.id();
        qa_sell = quantum_amount_usd(position.open_price.clone(), position.size.clone(), 6usize);
        qa_buy = quantum_amount(position.size.clone(), market.decimals());
        qa_fee = U512::from(((qa_sell.as_u128() as f64) * fee.parse::<f64>().expect("Incorrect fee given")).ceil() as u64);
    }
    else {
        asset_id_sell = market.id();
        if testnet { asset_id_buy = *USD_ID_TESTNET; }
        else { asset_id_buy = *USD_ID_MAINNET; }
        qa_sell = quantum_amount(position.size.clone(), market.decimals());
        qa_buy = quantum_amount_usd(position.open_price.clone(), position.size.clone(), 6usize);
        qa_fee = U512::from(((qa_buy.as_u128() as f64) * fee.parse::<f64>().expect("Incorrect fee given")).ceil() as u64);
    }
    let mut part_1 = qa_sell;
    part_1 <<= ORDER_FIELD_BIT_LENGTH_QA;
    part_1 += qa_buy;
    part_1 <<= ORDER_FIELD_BIT_LENGTH_QA;
    part_1 += qa_fee;
    part_1 <<= ORDER_FIELD_BIT_LENGTH_NONCE;
    part_1 += nonce.into();

    let mut part_2 = U512::from(ORDER_PREFIX);
    for _ in 0..3 {
        part_2 <<= ORDER_FIELD_BIT_LENGTH_POSITION;
        part_2 += U512::from(position_id);
    }
    part_2 <<= ORDER_FIELD_BIT_LENGTH_EXPIRATION;
    part_2 += U512::from(expiration_hours);
    part_2 <<= ORDER_PADDING_BITS;

    let asset_id_fee = {
        if testnet { *USD_ID_TESTNET }
        else { *USD_ID_MAINNET }
    };

    let assets_hash = pedersen_hash(
        (pedersen_hash((asset_id_sell, asset_id_buy)),
        asset_id_fee)
    );

    pedersen_hash(
        (pedersen_hash((assets_hash, part_1)),
        part_2)
    )
}

// Takes the order hash and STARK private key as input and outputs the order hash signature.
pub(crate) fn sign(hash: U512, privkey: U512) -> String {
    let (r, s) = sign_internal(hash, privkey);
    let mut hex_r = format!("{:x}", r);
    let mut hex_s = format!("{:x}", s);
    
    assert!(hex_r.len() <= 64);
    assert!(hex_s.len() <= 64);
    while hex_r.len() != 64 {
        hex_r = "0".to_string() + &hex_r;
    }
    while hex_s.len() != 64 {
        hex_s = "0".to_string() + &hex_s;
    }
    hex_r + &hex_s
}

// Internal sign function, used for actually performing the sign calculations.
pub(crate) fn sign_internal(hash: U512, privkey: U512) -> (U512, U512) {

    assert!(hash < *MAX_TO_SIGN);

    loop {
        let k_val = generate_k(privkey, hash);

        let x = ECMath::ec_mult(k_val, CONSTANT_POINTS[1], U512::from(ALPHA)).0;
        if x == U512::zero() || x > *MAX_TO_SIGN { continue; }
        let buffer = (hash + x * privkey) % *EC_ORDER;
        if buffer == U512::zero() { continue; }

        let inverse = ECMath::modinv(buffer, *EC_ORDER);
        let w = (k_val * inverse) % *EC_ORDER;
        if w == U512::zero() || w > *MAX_TO_SIGN { continue; }

        let s = ECMath::modinv(w, *EC_ORDER);

        return (x, s);
    }
}

// Nonce needed for creation of order hash.
fn convert_nonce(client_id: String) -> u128 {
    let mut hash = Sha256::new();
    hash.update(client_id);
    let hash = hash.finalize();
    let modulus = U512::from(NONCE_UPPER_BOUND_EXCLUSIVE);
    (U512::from(hash.as_slice()) % modulus).as_u128()
}


// Internal function for the creation of the pedersen hash.
fn pedersen_hash(element: (U512, U512)) -> U512 {

    // First Point
    let mut return_point: (U512, U512) = CONSTANT_POINTS[0];
    let mut x0 = element.0;
    assert!(x0 < *FIELD_PRIME);
    let point_list: [(U512, U512); N_ELEMENTS_BITS_HASH] = CONSTANT_POINTS[2..2+N_ELEMENTS_BITS_HASH].try_into().expect("Should never fail");
    for point in point_list {
        if x0 & U512::one() == U512::one() { return_point = ECMath::ec_add(return_point, point); }
        x0 >>= 1
    }
    assert!(x0 == U512::zero());

    // Second Point
    let mut x1 = element.1;
    assert!(x1 < *FIELD_PRIME);
    let point_list: [(U512, U512); N_ELEMENTS_BITS_HASH] = CONSTANT_POINTS[2+N_ELEMENTS_BITS_HASH..2+(2*N_ELEMENTS_BITS_HASH)].try_into().expect("Should never fail");
    for point in point_list {
        if x1 & U512::one() == U512::one() { return_point = ECMath::ec_add(return_point, point); }
        x1 >>= 1
    }
    assert!(x1 == U512::zero());

    return_point.0
}

// Converts the human readable amount to the quantum representation.
fn quantum_amount(human_amount: String, decimals: usize) -> U512 {
    let maybe_decimal = human_amount.find('.');
    if let Some(index) = maybe_decimal {
        let rest = human_amount[index+1..].len();
        assert!(rest <= decimals);
        let mut normalized = human_amount[0..index].to_string() + &human_amount[index+1..];
        let new_zeros = decimals - rest;
        for _ in 0..new_zeros {
            normalized.push('0');
        }
        U512::from_dec_str(&normalized).unwrap()
    }
    else {
        let mut normalized = human_amount;
        for _ in 0..decimals {
            normalized.push('0');
        }
        U512::from_dec_str(&normalized).unwrap() 
    }
}

// Comverts the human readable amount in USD to the quantum representation.
fn quantum_amount_usd(price: String, size: String, decimals: usize) -> U512 {
    let maybe_decimal_price = price.find('.');
    let maybe_decimal_size = size.find('.');
    let mut decimals_left = decimals;
    let mut normalized_price = price.clone();
    let mut normalized_size = size.clone();
    if let Some(i) = maybe_decimal_price {
        decimals_left -= price[i+1..].len();
        normalized_price = price[0..i].to_string() + &price[i+1..];
    }
    if let Some(i) = maybe_decimal_size {
        decimals_left -= size[i+1..].len();
        normalized_size = size[0..i].to_string() + &size[i+1..];
    }
    if decimals_left > 100 { return small_quantum_amount(price, size, decimals); }
    for _ in 0..decimals_left {
        normalized_price.push('0');
    }
    U512::from_dec_str(&normalized_price).unwrap() * U512::from_dec_str(&normalized_size).unwrap()
}

// Edge case for when human readable amount needs to be multiplied before converted.
fn small_quantum_amount(price: String, size: String, decimals: usize) -> U512 {
    let small_price = price.parse::<f64>().unwrap();
    let small_size = size.parse::<f64>().unwrap();
    let multiple = 10usize.pow(decimals as u32);
    U512::from(((small_price * multiple as f64) * small_size) as u128)
}

// Deterministically generates the ephemeral scalar k. Used for order signatures.
fn generate_k(privkey: U512, hash: U512) -> U512 {

    let mut message_hash = hash;
    let hash_bits = hash.bits();
    if hash_bits % 8 >= 1 && hash_bits % 8 <= 4 && hash_bits >= 248 { message_hash *= 16; }

    let mut privkey_bytes: [u8; 64] = [0u8; 64];
    privkey.to_big_endian(&mut privkey_bytes);

    let mut message_bytes: [u8; 64] = [0u8; 64];
    let message_hmac = crop_data(message_hash, ORDER_LEN.low_u32() as usize);
    message_hmac.to_big_endian(&mut message_bytes);

    let mut k = Hmac::<Sha256>::new_from_slice(&HMAC_SEED[..]).expect("Can't fail");
    k.update(&HMAC_BUFFER[..]);
    k.update(&privkey_bytes[32..]);
    k.update(&message_bytes[32..]);
    let k = k.finalize();

    let mut v = Hmac::<Sha256>::new_from_slice(&k.clone().into_bytes()).expect("Can't fail");
    v.update(&HMAC_BUFFER[0..32]);
    let v = v.finalize();

    let mut k = Hmac::<Sha256>::new_from_slice(&k.into_bytes()).expect("Should not fail.");
    let mut v_2 = v.clone().into_bytes().to_vec();
    v_2.push(1);
    k.update(v_2.as_slice());
    k.update(&privkey_bytes[32..]);
    k.update(&message_bytes[32..]);
    let mut k = k.finalize();

    let mut v_3 = Hmac::<Sha256>::new_from_slice(&k.clone().into_bytes()).expect("Should not fail.");
    v_3.update(&v.into_bytes());
    let mut v = v_3.finalize();

    loop {

        let mut t = Vec::new();

        while t.len() < 32 {
            let mut v_1 = Hmac::<Sha256>::new_from_slice(&k.clone().into_bytes()).expect("Should be fine.");
            v_1.update(&v.into_bytes());
            let v_1 = v_1.finalize();
            t.extend_from_slice(&v_1.clone().into_bytes());
            v = v_1;
        }

        let secret = crop_data(U512::from_big_endian(t.as_slice()), ORDER_LEN.low_u32() as usize);

        if secret != U512::zero() && secret < *EC_ORDER {
            return secret;
        }
        
        let mut k_inner = Hmac::<Sha256>::new_from_slice(&k.clone().into_bytes()).expect("Should be fine.");
        let mut v_temp = v.clone().into_bytes().to_vec();
        v_temp.push(0);
        k_inner.update(v_temp.as_slice());
        k = k_inner.finalize();

        let mut v_inner = Hmac::<Sha256>::new_from_slice(&k.clone().into_bytes()).expect("OK");
        v_inner.update(&v.clone().into_bytes());
        v = v_inner.finalize();
    }
}

// Utility for generate_k function
fn crop_data(data: U512, len: usize) -> U512 {
    let mut data_bits = data.bits();
    while data_bits % 8 != 0 {
        data_bits += 1;
    }
    if data_bits > len {
        data >> (data_bits - len)
    }
    else {
        data
    }
}

// Deprecated because it is super slow.
/*
fn generate_k_python(privkey_be_bytes: &[u8], hash_be_bytes: &[u8]) -> anyhow::Result<U512> {
    use pyo3::prelude::*;
    use num_bigint::BigUint;

    pyo3::prepare_freethreaded_python();
    let k = Python::with_gil(|py| -> anyhow::Result<String> {
        let generate_k = PyModule::import(py, "dydx3.starkex.starkex_resources.python_signature")?;
        let pk = BigUint::from_bytes_be(privkey_be_bytes);
        let hash = BigUint::from_bytes_be(hash_be_bytes);
        let k: BigUint = generate_k
            .getattr("generate_k_rfc6979")?
            .call1((hash,pk,))?
            .extract()?;
        Ok(k.to_str_radix(16))
    })?;

    Ok(U512::from_str_radix(&k, 16).unwrap())
}
*/


