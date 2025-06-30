use axum::Json;
use base64;
use serde::Deserialize;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::{initialize_mint, mint_to};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct TokenCreateRequest {
    mintAuthority: String,
    mint: String,
    pub decimals: i16, // accept even invalid values for validation
}

pub async fn create_token(Json(body): Json<TokenCreateRequest>) -> Json<serde_json::Value> {
    // ❗ Validate decimals range first
    if body.decimals < 0 || body.decimals > 255 {
        return Json(json!({
            "success": false,
            "error": "Invalid decimals: must be between 0 and 255"
        }));
    }

    let decimals = body.decimals as u8;

    let mint_pubkey = match Pubkey::from_str(&body.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid mint address"
            }));
        }
    };

    let authority = match Pubkey::from_str(&body.mintAuthority) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid mintAuthority address"
            }));
        }
    };

    let ix = match initialize_mint(&spl_token::id(), &mint_pubkey, &authority, None, decimals) {
        Ok(ix) => ix,
        Err(e) => {
            return Json(json!({
                "success": false,
                "error": format!("Failed to create instruction: {}", e),
            }));
        }
    };

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
    let mint = match Pubkey::from_str(&body.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid mint address"
            }));
        }
    };

    let destination = match Pubkey::from_str(&body.destination) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid destination address"
            }));
        }
    };

    let authority = match Pubkey::from_str(&body.authority) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid authority address"
            }));
        }
    };

    let ix = match mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[],
        body.amount,
    ) {
        Ok(ix) => ix,
        Err(e) => {
            return Json(json!({
                "success": false,
                "error": format!("Failed to create instruction: {}", e)
            }));
        }
    };

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
