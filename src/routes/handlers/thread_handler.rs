use actix_web::{get, web, HttpResponse};
use chrono::{DateTime, Utc};
use askama::Template;
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::utils::{app_state, jwt::Claims};

#[derive(Serialize, Deserialize)]
pub(crate) struct CreateThreadModel {
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

#[derive(Clone)]
struct ReplyView {
  id: i32,
  user_id: i32,
  body: String,
  created_at: String,
}

pub async fn create_thread(
  app_state: web::Data<app_state::AppState>,
  claim: Claims,
  thread_model: web::Form<CreateThreadModel>
) -> Result<HttpResponse, actix_web::Error> {
  let thread_entity = entity::thread::ActiveModel { 
    user_id: Set(claim.id), 
    title: Set(thread_model.title.clone()), 
    body: Set(thread_model.content.clone()),
    created_at: Set(Utc::now()),
    updated_at: Set(Utc::now()),
    ..Default::default()
  };

  thread_entity.insert(&app_state.db).await
    .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

  Ok(HttpResponse::SeeOther()
    .append_header(("Location", "/thread/thread-list"))
    .finish())
}

#[derive(Template)]
#[template(path = "new_thread.html")]
struct NewThreadTemplate {}
pub async fn create_thread_form() -> Result<HttpResponse, actix_web::Error> {
  let template = NewThreadTemplate {};
  let html = template.render()
    .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

  Ok(HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html))
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
  replies: Vec<ReplyView>,
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

  let replies: Vec<ReplyView> = entity::reply::Entity::find()
    .filter(entity::reply::Column::ThreadId.eq(thread_id))
    .all(&app_state.db).await
    .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?
    .into_iter()
    .map(|reply| ReplyView {
      id: reply.id,
      user_id: reply.user_id,
      body: reply.body,
      created_at: reply.created_at.to_rfc3339(),
    })
    .collect();

  let template = ThreadDetailTemplate { thread: thread_view, replies };
  let html = template.render()
    .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

  Ok(HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html))
}