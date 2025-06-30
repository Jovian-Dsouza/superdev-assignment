use crate::types::*;
use actix_web::dev::Payload;
use actix_web::FromRequest;
use actix_web::{error, post, web, HttpResponse};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use solana_sdk::signature::{Keypair, Signer};
use std::panic;

#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct SignMessageResponseData {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[post("/message/sign")]
pub async fn sign_message(req: web::Json<SignMessageRequest>) -> Result<HttpResponse, ApiError> {
    if req.message.is_empty() || req.secret.is_empty() {
        return Err(ApiError::BadRequest("Missing required fields".to_string()));
    }

    let keypair =
        panic::catch_unwind(|| Keypair::from_base58_string(&req.secret)).map_err(|_| {
            ApiError::BadRequest("Invalid base58 encoding for field 'secret'".to_string())
        })?;
    let signature = keypair.sign_message(req.message.as_bytes());

    let response_data = SignMessageResponseData {
        signature: base64::encode(signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: req.message.clone(),
    };

    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: response_data,
    }))
}

pub fn json_error_handler(
    err: error::JsonPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let resp = HttpResponse::BadRequest().json(serde_json::json!({
        "success": false,
        "error": format!("Missing required fields"),
    }));
    error::InternalError::from_response(err, resp).into()
}
