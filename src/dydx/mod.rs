use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use chrono::{Utc, DateTime};

mod market_helpers;
pub(crate) mod account;
pub(crate) mod constants;
pub(crate) mod crypto;
pub(crate) mod ec_math;

pub struct TradeData {
    pub trailing_percent: Option<String>,
    pub trigger_price: Option<String>,
    pub reduce_only: Option<bool>,
}

pub struct Position {
    open_price: String,
    size: String,
    side: Side,
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum OrderType {
    MARKET,
    LIMIT,
    STOP_LIMIT,
    TRAILING_STOP,
    TAKE_PROFIT,
}

pub struct InternalAccount {
    api_key: String,
    api_passphrase: String,
    api_secret: String,
    stark_private_key: String,
    ethereum_address: String,
    position_id: Option<u64>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Clone)]
pub enum Side {
    BUY,
    SELL,
}

#[derive(Copy, Clone)]
pub enum Exposure {
    Low,
    Medium,
    High,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Markets {
    BTC,
    ETH,
    SOL,
    FIL,
    AVAX,
    MATIC,
    NEAR,
    CRV,
    LINK,
    SNX,
    DOT,
    DOGE,
    ATOM,
    ENJ,
    AAVE,
    COMP,
    ALGO,
    ADA,
    ZEC,
    ZRX,
    EOS,
    ICP,
    TRX,
    XTZ,
    INCH,
    SUSHI,
    CELO,
    UNI,
    ETC,
    BCH,
    LTC,
    XMR,
    MKR,
    UMA,
    YFI,
    RUNE,
    XLM,
}

impl InternalAccount {

    pub fn new_uninitialized(api_key: String, api_passphrase: String, api_secret: String, stark_private_key: String, ethereum_address: String) -> Self {
        Self {
            api_key,
            api_passphrase,
            api_secret,
            stark_private_key,
            ethereum_address,
            position_id: None
        }
    }

    pub async fn onboard(&mut self, testnet: bool) -> anyhow::Result<()> {
        let position_id = self.find_position_id(testnet).await?;
        self.position_id = Some(position_id);
        Ok(())
    }

    pub(crate) fn signature_for(&self, request_path: String, method: String, data: Option<String>) -> (String, String) {

        let timestamp: DateTime<Utc> = Utc::now();
        let request = format!("{:?}{}{}", timestamp, method, request_path);
        let key = general_purpose::URL_SAFE_NO_PAD.decode(self.api_secret.clone()).unwrap();

        let mut mac = Hmac::<sha2::Sha256>::new_from_slice(&key).unwrap();
        mac.update(request.as_bytes());
        if let Some(d) = data {
            mac.update(d.as_bytes());
        }
    
        let signature = mac.finalize();
        let mut encoded_signature = general_purpose::URL_SAFE_NO_PAD.encode(signature.into_bytes());
        if encoded_signature.len() == 43 { encoded_signature.push('='); }
        (encoded_signature, format!("{:?}", timestamp))
    }

    pub(crate) fn stark_private_key(&self) -> primitive_types::U512 {
        primitive_types::U512::from_str_radix(&self.stark_private_key, 16).unwrap()
    }

    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn ethereum_address(&self) -> String {
        self.ethereum_address.clone()
    }
    
    pub fn position_id(&self) -> u64 {
        self.position_id.unwrap_or(0u64)
    }

}

impl Position {

    pub fn new(open_price: String, size: String, side: Side) -> Self {
        Self {
            open_price,
            size,
            side,
        }
    }

    pub fn price(&self) -> String {
        self.open_price.clone()
    }
}

impl TradeData {
    pub fn new(trailing_percent: Option<String>, trigger_price: Option<String>, reduce_only: Option<bool>) -> Self {
        Self {
            trailing_percent,
            trigger_price,
            reduce_only,
        }
    }
}
