use std::path;

use actix_web::{middleware::from_fn, web};
use crate::routes::middlewares;

use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/thread")
      .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
      .service(handlers::thread_handler::create_thread)
      .service(handlers::thread_handler::get_thread_list)
      .service(handlers::thread_handler::get_thread)
      .service(handlers::reply_handler::create_reply)
      .service(handlers::reply_handler::get_replies)
    );
}