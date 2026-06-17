use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, Statement, dynamic::Entity, EntityTrait};

use crate::utils::{ api_response, app_state::AppState };

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
  api_response::ApiResponse::new(200, format!("Hello, {}!", name).to_string())
}

#[get("/test")]
async fn test(app_state: web::Data<AppState>) -> impl Responder {
  let rows = entity::user::Entity::find()
    .all(&app_state.db)
    .await
    .unwrap();

  api_response::ApiResponse::new(
    200,
    serde_json::to_string(&rows).unwrap()
  )
}