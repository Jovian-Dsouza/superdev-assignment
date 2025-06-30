use std::str::FromStr;

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;
use crate::types::*;

#[derive(Deserialize)]
struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    mint_authority: String,
    mint: String,
    decimals: u8,
}

#[post("/token/create")]
pub async fn create_token(
    req: web::Json<CreateTokenRequest>,
) -> Result<HttpResponse, ApiError> {
    let mint_authority_pubkey = parse_pubkey(&req.mint_authority, "mintAuthority")?;
    let mint_pubkey = parse_pubkey(&req.mint, "mint")?;
    let token_program_id = spl_token::id();

    let instruction = spl_token::instruction::initialize_mint(
        &token_program_id,
        &mint_pubkey,
        &mint_authority_pubkey,
        None, // freeze
        req.decimals,
    )
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: InstructionResponseData::from(instruction),
    }))
}


fn parse_pubkey(key_str: &str, field_name: &str) -> Result<Pubkey, ApiError> {
    Pubkey::from_str(key_str).map_err(|_| {
        ApiError::BadRequest(format!(
            "Invalid base58 encoding for field '{}'",
            field_name
        ))
    })
}
