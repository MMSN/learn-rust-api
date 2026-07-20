use actix_web::{get, post, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, Set};

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

#[get("/thread-list")]
pub async fn get_thread_list(
  app_state: web::Data<app_state::AppState>,
  claim: Claims,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {

  let threads: Vec<ThreadModel> = entity::thread::Entity::find()
    .all(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
    .into_iter()
    .map(|thread| ThreadModel {
      id: thread.id,
      user_id: thread.user_id,
      title: thread.title,
      body: thread.body,
      created_at: thread.created_at,
      updated_at: thread.updated_at,
    }).collect();

  let res_string = serde_json::to_string(&threads)
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

  Ok(api_response::ApiResponse::new(
    200,
    res_string.to_owned()
  ))
} 