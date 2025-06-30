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
            "signature": base64::encode(signature),
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
    let signature = base64::decode(body.signature).unwrap();
    let pubkey = Pubkey::from_str(&body.pubkey).unwrap();

    let verified = pubkey
        .verify(
            body.message.as_bytes(),
            &solana_sdk::signature::Signature::new(&signature),
        )
        .is_ok();

    Json(json!({
        "success": true,
        "data": {
            "valid": verified,
            "message": body.message,
            "pubkey": body.pubkey
        }
    }))
}
