use actix_web::{post, web, HttpResponse, Responder};
use serde::Serialize;
use solana_sdk::signer::{keypair::Keypair, Signer};

use crate::types::*;

#[derive(Serialize)]
pub struct KeypairResponseData {
    pubkey: String,
    secret: String,
}

#[post("/keypair")]
pub async fn keypair() -> Result<HttpResponse, ApiError> {
    // Keypair::new() does not fail, but wrap in catch_unwind for robustness
    let pair = std::panic::catch_unwind(|| Keypair::new())
        .map_err(|_| ApiError::InternalError("Failed to generate keypair".to_string()))?;

    // to_base58_string should not fail, but handle just in case
    let pubkey = pair.pubkey().to_string();
    let secret = pair.to_base58_string();

    let response_data = KeypairResponseData { pubkey, secret };

    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: response_data,
    }))
}
