use axum::Json;
use base64;
use serde::Deserialize;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::initialize_mint;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct TokenCreateRequest {
    mintAuthority: String,
    mint: String,
    decimals: u8,
}

pub async fn create_token(Json(body): Json<TokenCreateRequest>) -> Json<serde_json::Value> {
    let mint_pubkey = Pubkey::from_str(&body.mint).unwrap();
    let authority = Pubkey::from_str(&body.mintAuthority).unwrap();

    let ix = initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &authority,
        None,
        body.decimals,
    )
    .unwrap();

    let accounts = ix
        .accounts
        .iter()
        .map(|acc| {
            json!({
                "pubkey": acc.pubkey.to_string(),
                "is_signer": acc.is_signer,
                "is_writable": acc.is_writable,
            })
        })
        .collect::<Vec<_>>();

    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": accounts,
            "instruction_data": base64::encode(ix.data),
        }
    }))
}

#[derive(Deserialize)]
pub struct TokenMintRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

pub async fn mint_token(Json(body): Json<TokenMintRequest>) -> Json<serde_json::Value> {
    let ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &Pubkey::from_str(&body.mint).unwrap(),
        &Pubkey::from_str(&body.destination).unwrap(),
        &Pubkey::from_str(&body.authority).unwrap(),
        &[],
        body.amount,
    )
    .unwrap();

    let accounts = ix
        .accounts
        .iter()
        .map(|acc| {
            json!({
                "pubkey": acc.pubkey.to_string(),
                "is_signer": acc.is_signer,
                "is_writable": acc.is_writable,
            })
        })
        .collect::<Vec<_>>();

    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": accounts,
            "instruction_data": base64::encode(ix.data),
        }
    }))
}
