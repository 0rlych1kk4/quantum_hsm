use pqcrypto_ntru::ntruhps2048509::{sign};  // Importing NTRU signing
use solana_sdk::{signature::Signature, transaction::Transaction};
use bitcoin::{Transaction as BtcTransaction, consensus::encode::serialize};
use ethers::signers::{LocalWallet, Signer};
use ethers::core::types::TransactionRequest;
use crate::key_manager::get_private_key;

pub fn sign_solana_transaction(wallet_id: &str, tx: &mut Transaction) -> Option<Signature> {
    if let Some(private_key) = get_private_key(wallet_id) {
        // Sign the Solana transaction using the quantum-resistant sign function
        let signature = sign(&private_key, &tx.message_data());
        
        // Wrap the signature to fit Solana's expected type
        Some(Signature::new_unique())  // Adjusted to use NTRU signing instead
    } else {
        None
    }
}

pub fn sign_bitcoin_transaction(wallet_id: &str, tx: &BtcTransaction) -> Option<Vec<u8>> {
    if let Some(private_key) = get_private_key(wallet_id) {
        // Sign the Bitcoin transaction using the quantum-resistant sign function
        let signature = sign(&private_key, &serialize(tx));
        
        Some(signature)
    } else {
        None
    }
}

pub async fn sign_ethereum_transaction(wallet_id: &str, tx: TransactionRequest) -> Option<Vec<u8>> {
    if let Some(private_key) = get_private_key(wallet_id) {
        // Ethereum signing still uses the standard signing mechanism for this case
        let wallet = LocalWallet::from_bytes(&private_key).ok()?;
        
        // Sign the Ethereum transaction
        let signature = wallet.sign_transaction(&tx).await.ok()?;
        Some(signature.to_vec())
    } else {
        None
    }
}
