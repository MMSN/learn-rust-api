use std::path;

use actix_web::{web};
use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/auth")
      .service(handlers::auth_handler::register_page)
      .service(handlers::auth_handler::register)
      .service(handlers::auth_handler::login_page)
      .service(handlers::auth_handler::login)
    );
} 