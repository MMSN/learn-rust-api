use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError, body::BoxBody, http::StatusCode, web};
use std::fmt::Display;

#[derive(Debug)]
pub struct ApiResponse {
  pub body: String,
  pub status_code: u16,
  response_code: StatusCode
}

impl ApiResponse {
  pub fn new(status_code: u16, body: String) -> Self {
    ApiResponse {
      status_code,
      body,
      response_code: StatusCode::from_u16(status_code).unwrap()
    }
  }
}

impl Responder for ApiResponse {
  type Body = BoxBody;

  fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
    let body: BoxBody = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
    HttpResponse::new(self.response_code).set_body(body)
  }
}

impl Display for ApiResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error: {} \n Status Code: {}", self.body, self.status_code)
  }
}

impl ResponseError for ApiResponse {
  fn status_code(&self) -> StatusCode {
    self.response_code
  }

  fn error_response(&self) -> HttpResponse<BoxBody> {
    let body: BoxBody = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
    HttpResponse::new(self.response_code).set_body(body)
  }
}