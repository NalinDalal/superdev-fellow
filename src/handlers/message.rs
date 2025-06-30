use axum::Json;
use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use serde_json::json;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct SignRequest {
    message: String,
    secret: String,
}

pub async fn sign_message(Json(body): Json<SignRequest>) -> Json<serde_json::Value> {
    let secret_bytes = bs58::decode(body.secret).into_vec().unwrap();
    let keypair = Keypair::from_bytes(&secret_bytes).unwrap();
    let signature = keypair.sign_message(body.message.as_bytes());

    Json(json!({
        "success": true,
        "data": {
            "signature": general_purpose::STANDARD.encode(signature),
            "public_key": keypair.pubkey().to_string(),
            "message": body.message
        }
    }))
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    message: String,
    signature: String,
    pubkey: String,
}

pub async fn verify_message(Json(body): Json<VerifyRequest>) -> Json<serde_json::Value> {
    // Decode the signature from base64
    let signature_bytes = general_purpose::STANDARD.decode(&body.signature).unwrap();

    // Use TryFrom to create Signature
    let signature = Signature::try_from(signature_bytes.as_slice()).unwrap();

    // Parse the public key
    let pubkey = Pubkey::from_str(&body.pubkey).unwrap();

    // Verify the signature against the message
    let is_valid = signature.verify(pubkey.as_ref(), body.message.as_bytes());

    Json(json!({
        "success": true,
        "data": {
            "valid": is_valid,
            "message": body.message,
            "pubkey": body.pubkey
        }
    }))
}
