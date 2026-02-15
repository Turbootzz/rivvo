use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::config::Config;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::UserResponse;
use crate::services::auth_service;
use crate::utils::jwt;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(
        min = 2,
        max = 255,
        message = "Name must be between 2 and 255 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 8,
        max = 128,
        message = "Password must be between 8 and 128 characters"
    ))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

pub async fn register(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = auth_service::register_user(pool.get_ref(), &body.email, &body.name, &body.password)
        .await?;

    let token = jwt::encode_token(user.id, &config.jwt_secret)
        .map_err(|e| AppError::InternalError(format!("Token generation failed: {e}")))?;

    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        user: user.into(),
    }))
}

pub async fn login(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = auth_service::login_user(pool.get_ref(), &body.email, &body.password).await?;

    let token = jwt::encode_token(user.id, &config.jwt_secret)
        .map_err(|e| AppError::InternalError(format!("Token generation failed: {e}")))?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: user.into(),
    }))
}

pub async fn me(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user = auth_service::get_user_by_id(pool.get_ref(), auth.user_id).await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}
