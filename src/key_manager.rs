use pqcrypto_dilithium::dilithium2::{keypair, PublicKey, SecretKey};
use pqcrypto_traits::sign::{PublicKey as _, SecretKey as TraitSecretKey};
use secrecy::{ExposeSecret, Secret}; //  works with secrecy v0.8
use std::collections::HashMap;
use parking_lot::Mutex;
use lazy_static::lazy_static;
use hex;

lazy_static! {
    static ref KEY_STORE: Mutex<HashMap<String, Secret<Vec<u8>>>> = Mutex::new(HashMap::new());
}

pub fn generate_dilithium_keypair(wallet_id: &str) -> String {
    let (public_key, secret_key) = keypair();

    let mut store = KEY_STORE.lock();
    store.insert(wallet_id.to_string(), Secret::new(secret_key.as_bytes().to_vec()));

    hex::encode(public_key.as_bytes())
}

pub fn get_private_key(wallet_id: &str) -> Option<SecretKey> {
    let store = KEY_STORE.lock();
    store.get(wallet_id).map(|secret_vec| {
        SecretKey::from_bytes(secret_vec.expose_secret()).expect("Invalid private key format")
    })
}

pub fn decode_public_key(hex_key: &str) -> Result<PublicKey, hex::FromHexError> {
    let bytes = hex::decode(hex_key)?;
    Ok(PublicKey::from_bytes(&bytes).unwrap())
}

