use actix_web::{get, post, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::utils::{api_response, app_state, jwt::Claims};

#[derive(Serialize, Deserialize)]
struct CreateReplyModel {
  content: String,
}

#[derive(Serialize, Deserialize)]
struct ReplyModel {
  pub id: i32,
  pub thread_id: i32,
  pub user_id: i32,
  pub parent_reply_id: Option<i32>,
  pub body: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[post("/{thread_id}/reply")]
pub async fn create_reply(
  app_state: web::Data<app_state::AppState>,
  claim: Claims,
  path: web::Path<i32>,
  reply_model: web::Json<CreateReplyModel>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
  let thread_id = path.into_inner();

  let thread_exists = entity::thread::Entity::find_by_id(thread_id)
    .one(&app_state.db)
    .await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

  if thread_exists.is_none() {
    return Err(api_response::ApiResponse::new(404, "Thread not found".to_string()));
  }

  let reply_entity = entity::reply::ActiveModel {
    thread_id: Set(thread_id),
    user_id: Set(claim.id),
    parent_reply_id: Set(None),
    body: Set(reply_model.content.clone()),
    created_at: Set(Utc::now()),
    updated_at: Set(Utc::now()),
    ..Default::default()
  };

  reply_entity.insert(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

  Ok(api_response::ApiResponse::new(
    200,
    format!("Reply created for thread {} by user {}", thread_id, claim.id)
  ))
}

#[get("/{thread_id}/replies")]
pub async fn get_replies(
  app_state: web::Data<app_state::AppState>,
  path: web::Path<i32>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
  let thread_id = path.into_inner();

  let replies: Vec<ReplyModel> = entity::reply::Entity::find()
    .filter(entity::reply::Column::ThreadId.eq(thread_id))
    .all(&app_state.db)
    .await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
    .into_iter()
    .map(|reply| ReplyModel {
      id: reply.id,
      thread_id: reply.thread_id,
      user_id: reply.user_id,
      parent_reply_id: reply.parent_reply_id,
      body: reply.body,
      created_at: reply.created_at,
      updated_at: reply.updated_at,
    })
    .collect();

  let res_string = serde_json::to_string(&replies)
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

  Ok(api_response::ApiResponse::new(200, res_string))
}
