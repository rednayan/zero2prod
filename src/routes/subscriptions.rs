use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(_from: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
