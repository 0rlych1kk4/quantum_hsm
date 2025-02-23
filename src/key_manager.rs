use pqcrypto_ntru::ntruhps2048509::{keypair, PublicKey};
use pqcrypto_traits::kem::PublicKey as _; // Import the PublicKey trait from kem module
use secrecy::{Secret, ExposeSecret};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use hex;

lazy_static! {
    static ref KEY_STORE: Mutex<HashMap<String, Secret<Vec<u8>>>> = Mutex::new(HashMap::new());
}

pub fn generate_ntru_keypair(wallet_id: &str) -> String {
    let (public_key, private_key) = keypair();
    
    // Storing the private key securely
    let mut keystore = KEY_STORE.lock().unwrap(); // The lock is applied correctly here
    keystore.insert(wallet_id.to_string(), Secret::new(private_key.to_vec()));
    
    // Convert public key to bytes and then hex string
    hex::encode(public_key.as_bytes()) // This should now work after importing the trait
}

pub fn get_private_key(wallet_id: &str) -> Option<Vec<u8>> {
    let keystore = KEY_STORE.lock().unwrap(); // The lock is applied correctly here
    keystore.get(wallet_id).map(|key| key.expose_secret().clone())
}

pub fn decode_public_key(hex_key: &str) -> Result<PublicKey, hex::FromHexError> {
    let bytes = hex::decode(hex_key)?; // Decode the hex string into bytes
    PublicKey::from_bytes(&bytes) // Use the from_bytes method to convert to PublicKey
}
