use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;
use sea_orm::{ConnectionTrait, Statement, dynamic::Entity, EntityTrait};

use crate::utils::{ api_response, app_state::AppState };

#[derive(Template)]
#[template(path = "home.html", escape = "html")]
struct HomeTemplate<'a> {
  title: &'a str,
  message: &'a str,
}

#[get("/")]
pub async fn index() -> impl Responder {
  let template = HomeTemplate {
    title: "Forum Home",
    message: "Welcome to your forum built with Actix Web and Askama.",
  };

  match template.render() {
    Ok(html) => HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(html),
    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
  }
}

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