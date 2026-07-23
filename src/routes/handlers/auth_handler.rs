use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use sea_orm::Condition;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use serde::Deserialize;
use serde::Serialize;
use sha256::digest;
use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::encode_jwt;
use crate::utils::{ api_response, app_state };

#[derive(Serialize, Deserialize)]
struct RegisterModel {
  name: String,
  email: String,
  password: String
}

#[derive(Serialize, Deserialize)]
struct LoginModel {
  email: String,
  password: String
}


#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {
  title: &'static str,
}
#[get("/register")]
pub async fn register_page() -> Result<HttpResponse, actix_web::Error> {
  let template = RegisterTemplate { title: "Register" };
  let html = template.render()
    .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

  Ok(HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html))
}

#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_data: web::Form<RegisterModel>
) -> Result<ApiResponse,ApiResponse>{

    let user_model = entity::user::ActiveModel {
        name: Set(register_data.name.clone()),
        email: Set(register_data.email.clone()),
        password: Set(digest(&register_data.password)),
        ..Default::default()
  }.insert(&app_state.db).await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;



  Ok(api_response::ApiResponse::new(200, format!("{}",user_model.id)))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
  title: &'static str,
}
#[get("/login")]
pub async fn login_page() -> Result<HttpResponse, actix_web::Error> {
  let template = LoginTemplate { title: "Login" };
  let html = template.render()
    .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

  Ok(HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html))
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_data: web::Form<LoginModel>
) -> Result<ApiResponse,ApiResponse> {

  let user_data = entity::user::Entity::find()
    .filter(
      Condition::all()
      .add(entity::user::Column::Email.eq(&login_data.email))
      .add(entity::user::Column::Password.eq(digest(&login_data.password)))
    ).one(&app_state.db).await
    .map_err(|err| ApiResponse::new(500,err.to_string()))?
    .ok_or(ApiResponse::new(404, "User Not Found".to_owned()))?;

    let token = encode_jwt(user_data.email, user_data.id)
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, format!("{{ 'token':'{}' }}",token)))
}