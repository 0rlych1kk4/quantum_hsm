use axum::{Json, Router, routing::post};
use serde::{Serialize, Deserialize};
use crate::key_manager::generate_dilithium_keypair;
use crate::signer::sign_message;
use tokio::net::TcpListener;
use hex;

#[derive(Serialize, Deserialize)]
struct KeyGenRequest {
    wallet_id: String,
}

#[derive(Serialize, Deserialize)]
struct SignRequest {
    wallet_id: String,
    message: String, // plain text or hex
    is_hex: bool,
}

async fn generate_key(Json(payload): Json<KeyGenRequest>) -> Json<String> {
    let public_key = generate_dilithium_keypair(&payload.wallet_id);
    Json(public_key)
}

async fn sign(Json(payload): Json<SignRequest>) -> Json<String> {
    let msg_bytes = if payload.is_hex {
        match hex::decode(&payload.message) {
            Ok(bytes) => bytes,
            Err(_) => return Json("Invalid hex message".to_string()),
        }
    } else {
        payload.message.into_bytes()
    };

    match sign_message(&payload.wallet_id, &msg_bytes) {
        Some(signature) => Json(hex::encode(signature)),
        None => Json("Signing failed".to_string()),
    }
}

pub async fn start_api_server() {
    let app = Router::new()
        .route("/generate", post(generate_key))
        .route("/sign", post(sign));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

