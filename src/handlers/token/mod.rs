use std::str::FromStr;

use crate::types::*;
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;

pub mod create;
pub mod mint;
pub mod transfer;

pub use create::*;
pub use mint::*;
pub use transfer::*;

pub fn parse_pubkey(key_str: &str, field_name: &str) -> Result<Pubkey, ApiError> {
    Pubkey::from_str(key_str).map_err(|_| {
        ApiError::BadRequest(format!(
            "Invalid base58 encoding for field '{}'",
            field_name
        ))
    })
}
