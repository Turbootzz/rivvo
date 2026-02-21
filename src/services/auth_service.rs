use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::models::user::User;
use crate::services::org_service;

pub async fn register_user(
    pool: &PgPool,
    email: &str,
    name: &str,
    password: &str,
) -> Result<User, AppError> {
    // Check if email already exists
    let existing: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest(
            "A user with this email already exists".to_string(),
        ));
    }

    let password = password.to_owned();
    let password_hash = tokio::task::spawn_blocking(move || hash_password(&password))
        .await
        .map_err(|e| AppError::InternalError(format!("Blocking task failed: {e}")))??;

    let user: User = sqlx::query_as(
        "INSERT INTO users (email, name, password_hash, provider) VALUES ($1, $2, $3, 'email') RETURNING *",
    )
    .bind(email)
    .bind(name)
    .bind(&password_hash)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            AppError::BadRequest("A user with this email already exists".to_string())
        }
        _ => AppError::DatabaseError(e),
    })?;

    // Auto-create an organization for the new user
    org_service::create_org(pool, &format!("{name}'s Workspace"), user.id).await?;

    Ok(user)
}

pub async fn login_user(pool: &PgPool, email: &str, password: &str) -> Result<User, AppError> {
    let user: User = sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    let stored_hash = user
        .password_hash
        .as_ref()
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?
        .clone();

    let password = password.to_owned();
    let valid = tokio::task::spawn_blocking(move || verify_password(&password, &stored_hash))
        .await
        .map_err(|e| AppError::InternalError(format!("Blocking task failed: {e}")))??;

    if !valid {
        return Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: uuid::Uuid) -> Result<User, AppError> {
    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(user)
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::InternalError(format!("Password hashing failed: {e}")))
}

fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::InternalError(format!("Invalid password hash: {e}")))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
