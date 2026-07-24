use actix_web::{Error, HttpMessage, body::MessageBody, dev::{ServiceRequest, ServiceResponse}, http::header::AUTHORIZATION, middleware::Next};

use crate::utils::{api_response::{self, ApiResponse}, jwt::decode_jwt};

pub async fn check_auth_middleware(
  req: ServiceRequest,
  next: Next<impl MessageBody>
) -> Result<ServiceResponse<impl MessageBody>, Error> {
  let token = match req.headers().get(AUTHORIZATION) {
    Some(auth) => auth
      .to_str()
      .ok()
      .and_then(|value| value.strip_prefix("Bearer ").map(str::to_owned))
      .ok_or_else(|| Error::from(api_response::ApiResponse::new(401, "Unauthorized".to_string())))?,
    None => {
      let cookie_header = req.headers().get("cookie")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

      cookie_header
        .split(';')
        .map(str::trim)
        .find_map(|part| part.strip_prefix("token=").map(str::to_owned))
        .ok_or_else(|| Error::from(api_response::ApiResponse::new(401, "Unauthorized".to_string())))?
    }
  };

  let claim = decode_jwt(token).map_err(|_| Error::from(api_response::ApiResponse::new(401, "Unauthorized".to_string())))?;
  req.extensions_mut().insert(claim.claims);

  next.call(req).await
    .map_err(|err| Error::from(ApiResponse::new(500, err.to_string())))
}