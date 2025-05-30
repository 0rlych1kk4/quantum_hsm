use pqcrypto_dilithium::dilithium2::{
    verify_detached_signature, PublicKey, DetachedSignature,
};
use pqcrypto_traits::sign::{PublicKey as _, DetachedSignature as _};

pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    let pk = match PublicKey::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    let sig = match DetachedSignature::from_bytes(signature) {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    verify_detached_signature(&sig, message, &pk).is_ok()
}

