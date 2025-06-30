use actix_web::{get, post};
use actix_web::{web, Responder};

use crate::types::AppState;

#[get("/")]
async fn get_health(data: web::Data<AppState>) -> impl Responder {
    format!("Server running as expected:  {}", data.app_name)
}
