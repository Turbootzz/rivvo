use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::changelog::ChangelogResponse;
use crate::services::{changelog_service, org_service};

#[derive(Deserialize, Validate)]
pub struct CreateChangelogRequest {
    #[validate(length(
        min = 1,
        max = 500,
        message = "Title must be between 1 and 500 characters"
    ))]
    pub title: String,
    #[validate(length(min = 1, message = "Body is required"))]
    pub body: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateChangelogRequest {
    #[validate(length(
        min = 1,
        max = 500,
        message = "Title must be between 1 and 500 characters"
    ))]
    pub title: String,
    #[validate(length(min = 1, message = "Body is required"))]
    pub body: String,
    pub is_draft: bool,
}

/// GET /api/orgs/{org_id}/changelog — admin list (includes drafts)
pub async fn list_entries(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    org_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let org_id = org_id.into_inner();
    org_service::require_org_admin(pool.get_ref(), org_id, auth.user_id).await?;
    let entries = changelog_service::get_all(pool.get_ref(), org_id).await?;
    let response: Vec<ChangelogResponse> =
        entries.into_iter().map(ChangelogResponse::from).collect();
    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/orgs/{org_id}/changelog
pub async fn create_entry(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    org_id: web::Path<Uuid>,
    body: web::Json<CreateChangelogRequest>,
) -> Result<HttpResponse, AppError> {
    let org_id = org_id.into_inner();
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    org_service::require_org_admin(pool.get_ref(), org_id, auth.user_id).await?;

    let entry = changelog_service::create(pool.get_ref(), org_id, &body.title, &body.body).await?;
    Ok(HttpResponse::Created().json(ChangelogResponse::from(entry)))
}

/// PUT /api/changelog/{id}
pub async fn update_entry(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    entry_id: web::Path<Uuid>,
    body: web::Json<UpdateChangelogRequest>,
) -> Result<HttpResponse, AppError> {
    let entry_id = entry_id.into_inner();
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let entry = changelog_service::get_by_id(pool.get_ref(), entry_id).await?;
    org_service::require_org_admin(pool.get_ref(), entry.org_id, auth.user_id).await?;

    // Set published_at on first publish
    let published_at = if !body.is_draft && entry.published_at.is_none() {
        Some(chrono::Utc::now())
    } else {
        entry.published_at
    };

    let updated = changelog_service::update(
        pool.get_ref(),
        entry_id,
        &body.title,
        &body.body,
        body.is_draft,
        published_at,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ChangelogResponse::from(updated)))
}

/// DELETE /api/changelog/{id}
pub async fn delete_entry(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    entry_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let entry_id = entry_id.into_inner();
    let entry = changelog_service::get_by_id(pool.get_ref(), entry_id).await?;
    org_service::require_org_admin(pool.get_ref(), entry.org_id, auth.user_id).await?;
    changelog_service::delete(pool.get_ref(), entry_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
