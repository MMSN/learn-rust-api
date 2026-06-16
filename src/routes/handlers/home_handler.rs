use actix_web::{get, web, Responder};

use crate::utils::api_response;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello, {}!", name).to_string())
}

#[get("/test")]
async fn test() -> impl Responder {
    api_response::ApiResponse::new(200, format!("Test !").to_string())
}