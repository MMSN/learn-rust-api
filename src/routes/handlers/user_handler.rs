use crate::utils::{api_response, app_state};
use actix_web::{get, web, Responder};

#[get("")]
pub async fn user(
  app_state: web::Data<app_state::AppState>
) -> impl Responder {

  api_response::ApiResponse::new(200, "User route".to_string())
}