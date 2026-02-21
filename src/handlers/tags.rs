use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::tag::TagResponse;
use crate::services::{board_service, org_service, post_service, tag_service};

#[derive(Deserialize, Validate)]
pub struct CreateTagRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Tag name must be between 1 and 100 characters"
    ))]
    pub name: String,
    pub color: Option<String>,
}

pub async fn list_tags(
    pool: web::Data<PgPool>,
    _auth: AuthenticatedUser,
    board_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let board_id = board_id.into_inner();
    let tags = tag_service::get_tags(pool.get_ref(), board_id).await?;
    let response: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();
    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_tag(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    board_id: web::Path<Uuid>,
    body: web::Json<CreateTagRequest>,
) -> Result<HttpResponse, AppError> {
    let board_id = board_id.into_inner();
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let board = board_service::get_board_by_id(pool.get_ref(), board_id).await?;
    org_service::require_org_admin(pool.get_ref(), board.org_id, auth.user_id).await?;

    let tag = tag_service::create_tag(pool.get_ref(), board_id, &body.name, body.color.as_deref())
        .await?;

    Ok(HttpResponse::Created().json(TagResponse::from(tag)))
}

pub async fn delete_tag(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    tag_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let tag_id = tag_id.into_inner();

    // Fetch tag to get board_id, then verify admin
    let tag: crate::models::tag::Tag = sqlx::query_as("SELECT * FROM tags WHERE id = $1")
        .bind(tag_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

    let board = board_service::get_board_by_id(pool.get_ref(), tag.board_id).await?;
    org_service::require_org_admin(pool.get_ref(), board.org_id, auth.user_id).await?;

    tag_service::delete_tag(pool.get_ref(), tag_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn add_tag_to_post(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (post_id, tag_id) = path.into_inner();

    let post = post_service::get_post_raw(pool.get_ref(), post_id).await?;
    let board = board_service::get_board_by_id(pool.get_ref(), post.board_id).await?;
    org_service::require_org_admin(pool.get_ref(), board.org_id, auth.user_id).await?;

    tag_service::add_tag_to_post(pool.get_ref(), post_id, tag_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn remove_tag_from_post(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (post_id, tag_id) = path.into_inner();

    let post = post_service::get_post_raw(pool.get_ref(), post_id).await?;
    let board = board_service::get_board_by_id(pool.get_ref(), post.board_id).await?;
    org_service::require_org_admin(pool.get_ref(), board.org_id, auth.user_id).await?;

    tag_service::remove_tag_from_post(pool.get_ref(), post_id, tag_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
