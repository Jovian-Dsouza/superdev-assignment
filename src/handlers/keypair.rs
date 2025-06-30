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
pub async fn keypair() -> impl Responder {
    let pair = Keypair::new();

    // println!("Pubkey:\n{}\n", &pair.pubkey().to_string());
    // println!("Base58 private key:\n{}\n", &pair.to_base58_string());
    // println!("JSON private key:\n{:?}", &pair.to_bytes());

    let response_data = KeypairResponseData {
        pubkey: pair.pubkey().to_string(),
        secret: pair.to_base58_string(),
    };
    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: response_data,
    })
    
}
