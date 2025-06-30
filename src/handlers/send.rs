#[derive(Deserialize)]
pub struct SolTransferRequest {
    from: String,
    to: String,
    lamports: u64,
}

pub async fn send_sol(Json(body): Json<SolTransferRequest>) -> Json<serde_json::Value> {
    let ix = solana_sdk::system_instruction::transfer(
        &Pubkey::from_str(&body.from).unwrap(),
        &Pubkey::from_str(&body.to).unwrap(),
        body.lamports,
    );

    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": ix.accounts.iter().map(|acc| acc.pubkey.to_string()).collect::<Vec<_>>(),
            "instruction_data": base64::encode(ix.data),
        }
    }))
}
pub struct TokenSendRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}

pub async fn send_token(Json(body): Json<TokenSendRequest>) -> Json<serde_json::Value> {
    let ix = spl_token::instruction::transfer(
        &spl_token::id(),
        &Pubkey::from_str(&body.owner).unwrap(),
        &Pubkey::from_str(&body.destination).unwrap(),
        &Pubkey::from_str(&body.owner).unwrap(),
        &[],
        body.amount,
    ).unwrap();

    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": ix.accounts.iter().map(|acc| json!({
                "pubkey": acc.pubkey.to_string(),
                "isSigner": acc.is_signer,
            })).collect::<Vec<_>>(),
            "instruction_data": base64::encode(ix.data),
        }
    }))
}

