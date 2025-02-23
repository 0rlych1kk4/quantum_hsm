use axum::{Json, Router, routing::{post}};
use serde::{Serialize, Deserialize};
use tokio::net::TcpListener;
use crate::key_manager::generate_ntru_keypair;
use crate::signer::{sign_solana_transaction, sign_bitcoin_transaction, sign_ethereum_transaction};
use solana_sdk::transaction::Transaction;
use bitcoin::consensus::encode::deserialize;
use ethers::core::types::TransactionRequest;
use hex;
use bincode;

#[derive(Serialize, Deserialize)]
struct KeyGenRequest {
    wallet_id: String,
}

#[derive(Serialize, Deserialize)]
struct SignRequest {
    wallet_id: String,
    blockchain: String,
    transaction_data: String,
}

async fn generate_key(Json(payload): Json<KeyGenRequest>) -> Json<String> {
    let public_key = generate_ntru_keypair(&payload.wallet_id);
    Json(public_key)
}

async fn sign_transaction(Json(payload): Json<SignRequest>) -> Json<String> {
    match payload.blockchain.as_str() {
        "solana" => {
            let decoded_data = match hex::decode(&payload.transaction_data) {
                Ok(data) => data,
                Err(_) => return Json("Invalid hex encoding for Solana transaction".to_string()),
            };

            let mut tx: Transaction = match bincode::deserialize(&decoded_data) {
                Ok(tx) => tx,
                Err(_) => return Json("Failed to deserialize Solana transaction".to_string()),
            };

            match sign_solana_transaction(&payload.wallet_id, &mut tx) {
                Some(sig) => Json(sig.to_string()),
                None => Json("Solana transaction signing failed".to_string()),
            }
        }

        "bitcoin" => {
            let decoded_data = match hex::decode(&payload.transaction_data) {
                Ok(data) => data,
                Err(_) => return Json("Invalid hex encoding for Bitcoin transaction".to_string()),
            };

            let tx = match deserialize(&decoded_data) {
                Ok(tx) => tx,
                Err(_) => return Json("Failed to deserialize Bitcoin transaction".to_string()),
            };

            match sign_bitcoin_transaction(&payload.wallet_id, &tx) {
                Some(sig) => Json(hex::encode(sig)),
                None => Json("Bitcoin transaction signing failed".to_string()),
            }
        }

        "ethereum" => {
            let tx = TransactionRequest::default();

            match sign_ethereum_transaction(&payload.wallet_id, tx).await {
                Some(sig) => Json(hex::encode(sig)),
                None => Json("Ethereum transaction signing failed".to_string()),
            }
        }

        _ => Json("Unsupported Blockchain".to_string()),
    }
}

pub async fn start_api_server() {
    let app = Router::new()
        .route("/generate", post(generate_key))
        .route("/sign", post(sign_transaction));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
