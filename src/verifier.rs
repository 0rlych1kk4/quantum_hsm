use pqcrypto_dilithium::dilithium2::{verify, PublicKey, DetachedSignature};
use pqcrypto_traits::sign::{PublicKey as _, DetachedSignature as _};

pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    let pk = PublicKey::from_bytes(public_key).ok()?;
    let sig = DetachedSignature::from_bytes(signature).ok()?;
    verify(message, &sig, &pk).is_ok()
}

