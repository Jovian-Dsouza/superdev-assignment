use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use crate::types::*;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[post("/token/create")]
pub async fn create_token(
    req: web::Json<CreateTokenRequest>,
) -> Result<HttpResponse, ApiError> {
    info!(?req, "Create token");
    let mint_authority_pubkey = super::parse_pubkey(&req.mint_authority, "mintAuthority")?;
    let mint_pubkey = super::parse_pubkey(&req.mint, "mint")?;
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