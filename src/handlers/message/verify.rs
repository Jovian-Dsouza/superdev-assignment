use crate::types::*;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponseData {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[post("/message/verify")]
pub async fn verify_message(
    req: web::Json<VerifyMessageRequest>,
) -> Result<HttpResponse, ApiError> {
    if req.message.is_empty() || req.signature.is_empty() || req.pubkey.is_empty() {
        return Err(ApiError::BadRequest("Missing required fields".to_string()));
    }

    let pubkey = Pubkey::from_str(&req.pubkey).map_err(|_| {
        ApiError::BadRequest("Invalid base58 encoding for field 'pubkey'".to_string())
    })?;
    let signature_bytes = base64::decode(&req.signature).map_err(|_| {
        ApiError::BadRequest("Invalid base64 encoding for field 'signature'".to_string())
    })?;
    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| ApiError::BadRequest("Signature must be 64 bytes".to_string()))?;

    let valid = signature.verify(pubkey.as_ref(), req.message.as_bytes());

    let response_data = VerifyMessageResponseData {
        valid,
        message: req.message.clone(),
        pubkey: req.pubkey.clone(),
    };

    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: response_data,
    }))
}
