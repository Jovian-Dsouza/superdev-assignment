use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use crate::types::*;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[post("/token/mint")]
pub async fn mint_token(
    req: web::Json<MintTokenRequest>,
) -> Result<HttpResponse, ApiError> {
    info!(?req, "mint token");
    let mint_pubkey = super::parse_pubkey(&req.mint, "mint")?;
    let destination_pubkey = super::parse_pubkey(&req.destination, "destination")?;
    let authority_pubkey = super::parse_pubkey(&req.authority, "authority")?;
    let token_program_id = spl_token::id();

    let instruction = spl_token::instruction::mint_to(
        &token_program_id,
        &mint_pubkey,
        &destination_pubkey,
        &authority_pubkey,
        &[], // No multisig signers
        req.amount,
    )
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: InstructionResponseData::from(instruction),
    }))
}
