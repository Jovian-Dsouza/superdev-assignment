use actix_web::{web, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use solana_client::nonblocking::rpc_client::RpcClient;

mod handlers;
use handlers::*;

mod types;
use types::AppState;

fn configure_services(cfg: &mut web::ServiceConfig) {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    cfg.app_data(web::Data::new(AppState {
        app_name: String::from("Superdev assingment"),
        rpc_client: client,
    }))
    .service(get_health::get_health)
    .service(keypair::keypair)
    .service(token::create_token)
    .service(token::mint_token)
    .service(token::transfer_token)
    .service(message::sign_message);
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    let config = move |cfg: &mut ServiceConfig| {
        configure_services(cfg);
    };

    Ok(config.into())
}
