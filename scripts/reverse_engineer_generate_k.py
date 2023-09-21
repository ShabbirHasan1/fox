import hashlib
import math
import hmac
import binascii
from binascii import hexlify

import os
from dotenv import load_dotenv

load_dotenv()

key = int(os.getenv('STARK_PRIVATE_KEY'), 16)
msg = 2779906671990622424237868443552817138689064617526907728512728741499664861624
EC_ORDER = 3618502788666131213697322783095070105526743751716087489154079457884512865583

def number_to_string_crop(num, order):
    l = orderlen(order)
    fmt_str = "%0" + str(2 * l) + "x"
    string = binascii.unhexlify((fmt_str % num).encode())
    return string[:l]

def hmac_compat(ret):
        return ret

def bit_length(x):
        return x.bit_length() or 1

def entropy_to_bits(ent_256):
        """Convert a bytestring to string of 0's and 1's"""
        return "".join(bin(ord(x))[2:].zfill(8) for x in ent_256)

def number_to_string(num, order):
    l = orderlen(order)
    fmt_str = "%0" + str(2 * l) + "x"
    string = binascii.unhexlify((fmt_str % num).encode())

    assert len(string) == l, (len(string), l)
    return string


def orderlen(order):
    return (1 + len("%x" % order)) // 2  # bytes

def bits2octets(data, order):
    z1 = bits2int(data, bit_length(order))
    z2 = z1 - order

    if z2 < 0:
        z2 = z1

    return number_to_string_crop(z2, order)

def bits2int(data, qlen):
    x = int(hexlify(data), 16)
    l = len(data) * 8
    if l > qlen:
        return x >> (l - qlen)
    return x


def generate_k(order, secexp, hash_func, data, retry_gen=0, extra_entropy=b""):
    """
    Generate the ``k`` value - the nonce for DSA.

    :param int order: order of the DSA generator used in the signature
    :param int secexp: secure exponent (private key) in numeric form
    :param hash_func: reference to the same hash function used for generating
        hash, like :py:class:`hashlib.sha1`
    :param bytes data: hash in binary form of the signing data
    :param int retry_gen: how many good 'k' values to skip before returning
    :param bytes extra_entropy: additional added data in binary form as per
        section-3.6 of rfc6979
    :rtype: int
    """

    qlen = bit_length(order)
    holen = hash_func().digest_size
    rolen = (qlen + 7) // 8

    bx = (
        hmac_compat(number_to_string(secexp, order)),
        hmac_compat(bits2octets(data, order)),
        hmac_compat(extra_entropy),
    )

    # Step B
    v = b"\x01" * holen

    # Step C
    k = b"\x00" * holen

    # Step D

    k = hmac.new(k, digestmod=hash_func)
    k.update(v + b"\x00")
    for i in bx:
        k.update(i)
    k = k.digest()

    # Step E
    v = hmac.new(k, v, hash_func).digest()

    # Step F
    k = hmac.new(k, digestmod=hash_func)
    k.update(v + b"\x01")
    for i in bx:
        k.update(i)
    k = k.digest()

    # Step G
    v = hmac.new(k, v, hash_func).digest()

    # Step H
    while True:
        # Step H1
        t = b""

        # Step H2
        while len(t) < rolen:
            v = hmac.new(k, v, hash_func).digest()
            t += v

        # Step H3
        secret = bits2int(t, qlen)

        if 1 <= secret < order:
            if retry_gen <= 0:
                return secret
            retry_gen -= 1

        k = hmac.new(k, v + b"\x00", hash_func).digest()
        v = hmac.new(k, v, hash_func).digest()


def generate_k_rfc6979(msg_hash: int, priv_key: int) -> int:
    # Pad the message hash, for consistency with the elliptic.js library.
    print("Hash: " + str(msg_hash))
    print("Key: " + str(priv_key))
    if 1 <= msg_hash.bit_length() % 8 <= 4 and msg_hash.bit_length() >= 248:
        # Only if we are one-nibble short:
        msg_hash *= 16

    extra_entropy = b''

    return generate_k(EC_ORDER, priv_key, hashlib.sha256,
                      msg_hash.to_bytes(math.ceil(msg_hash.bit_length() / 8), 'big'),
                      extra_entropy=extra_entropy)

print('Expected K: ' + str(generate_k_rfc6979(msg, key)));
