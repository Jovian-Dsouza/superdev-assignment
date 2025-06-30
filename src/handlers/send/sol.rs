use crate::types::*;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, system_instruction};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Serialize)]
struct SolTransferResponseData {
    program_id: String,
    accounts: Vec<String>,
    instruction_data: String,
}

#[post("/send/sol")]
pub async fn send_sol(req: web::Json<SendSolRequest>) -> Result<HttpResponse, ApiError> {
    if req.from.is_empty() || req.to.is_empty() {
        return Err(ApiError::BadRequest("Missing required fields".to_string()));
    }
    if req.lamports == 0 {
        return Err(ApiError::BadRequest(
            "Lamports must be greater than zero".to_string(),
        ));
    }
    let from_pubkey = Pubkey::from_str(&req.from).map_err(|_| {
        ApiError::BadRequest("Invalid base58 encoding for field 'from'".to_string())
    })?;
    let to_pubkey = Pubkey::from_str(&req.to)
        .map_err(|_| ApiError::BadRequest("Invalid base58 encoding for field 'to'".to_string()))?;

    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, req.lamports);

    let response_data = SolTransferResponseData {
        program_id: instruction.program_id.to_string(),
        accounts: instruction
            .accounts
            .into_iter()
            .map(|acc| acc.pubkey.to_string())
            .collect(),
        instruction_data: base64::encode(instruction.data),
    };

    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: response_data,
    }))
}
