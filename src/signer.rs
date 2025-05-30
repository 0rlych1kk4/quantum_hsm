use pqcrypto_dilithium::dilithium2::{sign, SignedMessage};
use pqcrypto_traits::sign::SignedMessage as TraitSignedMessage;
use crate::key_manager::get_private_key;

pub fn sign_message(wallet_id: &str, message: &[u8]) -> Option<Vec<u8>> {
    if let Some(secret_key) = get_private_key(wallet_id) {
        let signature: SignedMessage = sign(message, &secret_key);
        Some(signature.as_bytes().to_vec())
    } else {
        None
    }
}

