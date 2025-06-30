use crate::types::*;
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(Deserialize)]
pub struct TransferTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[post("/send/token")]
pub async fn transfer_token(
    req: web::Json<TransferTokenRequest>,
) -> Result<HttpResponse, ApiError> {
    let destination_pubkey = super::parse_pubkey(&req.destination, "destination")?;
    let mint_pubkey = super::parse_pubkey(&req.mint, "mint")?;
    let owner_pubkey = super::parse_pubkey(&req.owner, "owner")?;
    let token_program_id = spl_token::id();

    // Derive the owner's associated token account (ATA)
    let source_pubkey =
        spl_associated_token_account::get_associated_token_address(&owner_pubkey, &mint_pubkey);

    let instruction = spl_token::instruction::transfer(
        &token_program_id,
        &source_pubkey,
        &destination_pubkey,
        &owner_pubkey,
        &[], // No multisig signers
        req.amount,
    )
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: InstructionResponseData::from(instruction),
    }))
}
