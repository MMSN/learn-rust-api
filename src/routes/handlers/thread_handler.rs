use actix_web::{get, post, web, HttpResponse};
use chrono::{DateTime, Utc};
use askama::Template;
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::utils::{api_response, app_state, jwt::Claims};

#[derive(Serialize, Deserialize)]
struct CreateThreadModel {
  title: String,
  content: String,
}

#[derive(Serialize, Deserialize)]
struct ThreadModel {
  pub id: i32,
  pub user_id: i32,
  pub title: String,
  pub body: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
struct ThreadView {
  id: i32,
  user_id: i32,
  title: String,
  body: String,
  created_at: String,
  updated_at: String,
}

#[post("/create")]
pub async fn create_thread(
  app_state: web::Data<app_state::AppState>,
  claim: Claims,
  thread_model: web::Json<CreateThreadModel>
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {

  let thread_entity = entity::thread::ActiveModel { 
    user_id: Set(claim.id), 
    title: Set(thread_model.title.clone()), 
    body: Set(thread_model.content.clone()),
    created_at: Set(Utc::now()),
    updated_at: Set(Utc::now()),
    ..Default::default()
  };

  thread_entity.insert(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

  Ok(api_response::ApiResponse::new(
    200,
    format!("Thread created by user {} with title: {}", claim.id, thread_model.title)
  ))
}

#[derive(Template)]
#[template(path = "thread_list.html")]
struct ThreadListTemplate {
    //threads: &'a [Thread],
    threads: Vec<ThreadView>,
}
#[get("/thread-list")]
pub async fn get_thread_list(
  app_state: web::Data<app_state::AppState>,
  _claim: Claims,
) -> Result<HttpResponse, actix_web::Error> {

  let threads: Vec<ThreadView> = entity::thread::Entity::find()
    .all(&app_state.db).await
    .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?
    .into_iter()
    .map(|thread| ThreadView {
      id: thread.id,
      user_id: thread.user_id,
      title: thread.title,
      body: thread.body,
      created_at: thread.created_at.to_rfc3339(),
      updated_at: thread.updated_at.to_rfc3339(),
    }).collect();

  let template = ThreadListTemplate { threads };
  let html = template.render()
    .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

  Ok(HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html))
}

#[derive(Template)]
#[template(path = "thread_detail.html")]
struct ThreadDetailTemplate {
  thread: ThreadView,
}
#[get("/{thread_id}")]
pub async fn get_thread(
  app_state: web::Data<app_state::AppState>,
  _claim: Claims,
  thread_id: web::Path<i32>
) -> Result<HttpResponse, actix_web::Error> {

  let thread_id = thread_id.into_inner();

  let thread = entity::thread::Entity::find()
    .filter(entity::thread::Column::Id.eq(thread_id))
    .one(&app_state.db).await
    .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

  let Some(thread) = thread else {
    return Ok(HttpResponse::NotFound().body("Thread not found"));
  };

  let thread_view = ThreadView {
    id: thread.id,
    user_id: thread.user_id,
    title: thread.title,
    body: thread.body,
    created_at: thread.created_at.to_rfc3339(),
    updated_at: thread.updated_at.to_rfc3339(),
  };

  let template = ThreadDetailTemplate { thread: thread_view };
  let html = template.render()
    .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

  Ok(HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html))
}