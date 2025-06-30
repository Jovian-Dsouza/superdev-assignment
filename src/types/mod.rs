pub mod app_state;
use std::fmt;

use actix_web::{HttpResponse, ResponseError};
pub use app_state::*;
use serde::Serialize;
use solana_sdk::instruction::Instruction;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    InternalError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_message) = match self {
            ApiError::BadRequest(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                msg.to_string(),
            ),
            ApiError::InternalError(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("An internal error occurred: {}", msg),
            ),
        };

        HttpResponse::build(status_code).json(ErrorResponse {
            success: false,
            error: error_message,
        })
    }
}


#[derive(Serialize)]
pub struct InstructionAccount {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}


#[derive(Serialize)]
pub struct InstructionResponseData {
    program_id: String,
    accounts: Vec<InstructionAccount>,
    instruction_data: String,
}

// impl From<Instruction> for InstructionResponseData {
//     fn from(instruction: Instruction) -> Self {
//         InstructionResponseData {
//             program_id: instruction.program_id.to_string(),
//             accounts: instruction
//                 .accounts
//                 .into_iter()
//                 .map(|meta| InstructionAccount {
//                     pubkey: meta.pubkey.to_string(),
//                     is_signer: meta.is_signer,
//                     is_writable: meta.is_writable,
//                 })
//                 .collect(),
//             //TODO: use Engine::encode
//             instruction_data: base64::encode(instruction.data),
//         }
//     }
// }

impl From<Instruction> for InstructionResponseData {
    fn from(instruction: Instruction) -> Self {
        let serialized_data = bincode::serialize(&instruction.data)
            .unwrap_or_else(|err| {
                eprintln!("Failed to serialize instruction data: {}", err);
                Vec::new()
            });

        InstructionResponseData {
            program_id: instruction.program_id.to_string(),
            accounts: instruction
                .accounts
                .into_iter()
                .map(|meta| InstructionAccount {
                    pubkey: meta.pubkey.to_string(),
                    is_signer: meta.is_signer,
                    is_writable: meta.is_writable,
                })
                .collect(),
            instruction_data: base64::encode(serialized_data),
        }
    }
}