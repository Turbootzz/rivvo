use actix_web::dev::Payload;
use actix_web::{
    Error, FromRequest, HttpRequest, error::ErrorInternalServerError, error::ErrorUnauthorized,
};
use serde_json::json;
use std::future::{Ready, ready};
use uuid::Uuid;

use crate::config::Config;
use crate::utils::jwt;

pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let result = extract_user(req);
        ready(result)
    }
}

fn extract_user(req: &HttpRequest) -> Result<AuthenticatedUser, Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ErrorUnauthorized(json!({ "error": "Missing Authorization header" })))?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        ErrorUnauthorized(json!({ "error": "Invalid Authorization header format" }))
    })?;

    let config = req
        .app_data::<actix_web::web::Data<Config>>()
        .ok_or_else(|| {
            ErrorInternalServerError(json!({ "error": "Server configuration error" }))
        })?;

    let claims = jwt::decode_token(token, &config.jwt_secret)
        .map_err(|_| ErrorUnauthorized(json!({ "error": "Invalid or expired token" })))?;

    Ok(AuthenticatedUser {
        user_id: claims.sub,
    })
}
