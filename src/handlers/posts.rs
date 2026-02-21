use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::post::{PostAuthor, PostDetailResponse, PostListResponse};
use crate::models::tag::TagResponse;
use crate::services::{board_service, org_service, post_service, tag_service};

#[derive(Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(
        min = 1,
        max = 500,
        message = "Title must be between 1 and 500 characters"
    ))]
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdatePostRequest {
    #[validate(length(
        min = 1,
        max = 500,
        message = "Title must be between 1 and 500 characters"
    ))]
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

#[derive(Deserialize)]
pub struct PostQuery {
    pub sort: Option<String>,
    pub status: Option<String>,
}

pub async fn list_posts(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    board_id: web::Path<Uuid>,
    query: web::Query<PostQuery>,
) -> Result<HttpResponse, AppError> {
    let board_id = board_id.into_inner();
    let sort = query.sort.as_deref().unwrap_or("votes");
    let status_filter = query.status.as_deref();

    // Verify user is org member
    let board = board_service::get_board_by_id(pool.get_ref(), board_id).await?;
    org_service::get_member(pool.get_ref(), board.org_id, auth.user_id).await?;

    let rows = post_service::get_posts(pool.get_ref(), board_id, auth.user_id, sort, status_filter)
        .await?;

    // Fetch tags for all posts in batch
    let mut response: Vec<PostListResponse> = Vec::with_capacity(rows.len());
    for row in rows {
        let tags = tag_service::get_post_tags(pool.get_ref(), row.id).await?;
        let tag_responses: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();

        let description_preview = row.description.as_ref().map(|d| {
            if d.chars().count() > 200 {
                let truncated: String = d.chars().take(200).collect();
                format!("{truncated}...")
            } else {
                d.clone()
            }
        });

        response.push(PostListResponse {
            id: row.id,
            title: row.title,
            description_preview,
            status: row.status.unwrap_or_else(|| "open".to_string()),
            vote_count: row.vote_count.unwrap_or(0),
            comment_count: row.comment_count.unwrap_or(0),
            pinned: row.pinned.unwrap_or(false),
            author_name: row.author_name,
            has_voted: row.has_voted,
            tags: tag_responses,
            created_at: row.created_at,
        });
    }

    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_post(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    board_id: web::Path<Uuid>,
    body: web::Json<CreatePostRequest>,
) -> Result<HttpResponse, AppError> {
    let board_id = board_id.into_inner();
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Verify board exists and user is org member
    let board = board_service::get_board_by_id(pool.get_ref(), board_id).await?;
    org_service::get_member(pool.get_ref(), board.org_id, auth.user_id).await?;

    let post = post_service::create_post(
        pool.get_ref(),
        board_id,
        auth.user_id,
        &body.title,
        body.description.as_deref(),
    )
    .await?;

    // Return full detail response
    let detail = post_service::get_post(pool.get_ref(), post.id, auth.user_id).await?;
    let tags = tag_service::get_post_tags(pool.get_ref(), post.id).await?;

    Ok(HttpResponse::Created().json(build_detail_response(detail, tags)))
}

pub async fn get_post(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_board_id, post_id) = path.into_inner();

    let detail = post_service::get_post(pool.get_ref(), post_id, auth.user_id).await?;
    let tags = tag_service::get_post_tags(pool.get_ref(), post_id).await?;

    Ok(HttpResponse::Ok().json(build_detail_response(detail, tags)))
}

/// Direct post lookup by ID — no board_id required in the path.
pub async fn get_post_direct(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = post_id.into_inner();

    // Verify org membership through post -> board -> org chain
    let post = post_service::get_post_raw(pool.get_ref(), post_id).await?;
    let board = board_service::get_board_by_id(pool.get_ref(), post.board_id).await?;
    org_service::get_member(pool.get_ref(), board.org_id, auth.user_id).await?;

    let detail = post_service::get_post(pool.get_ref(), post_id, auth.user_id).await?;
    let tags = tag_service::get_post_tags(pool.get_ref(), post_id).await?;

    Ok(HttpResponse::Ok().json(build_detail_response(detail, tags)))
}

pub async fn update_post(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdatePostRequest>,
) -> Result<HttpResponse, AppError> {
    let (_board_id, post_id) = path.into_inner();
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let post = post_service::get_post_raw(pool.get_ref(), post_id).await?;

    // Only author or org admin can update
    if post.author_id != Some(auth.user_id) {
        let board = board_service::get_board_by_id(pool.get_ref(), post.board_id).await?;
        org_service::require_org_admin(pool.get_ref(), board.org_id, auth.user_id).await?;
    }

    let updated = post_service::update_post(
        pool.get_ref(),
        post_id,
        &body.title,
        body.description.as_deref(),
    )
    .await?;

    let detail = post_service::get_post(pool.get_ref(), updated.id, auth.user_id).await?;
    let tags = tag_service::get_post_tags(pool.get_ref(), updated.id).await?;

    Ok(HttpResponse::Ok().json(build_detail_response(detail, tags)))
}

pub async fn delete_post(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_board_id, post_id) = path.into_inner();

    let post = post_service::get_post_raw(pool.get_ref(), post_id).await?;

    // Only author or org admin can delete
    if post.author_id != Some(auth.user_id) {
        let board = board_service::get_board_by_id(pool.get_ref(), post.board_id).await?;
        org_service::require_org_admin(pool.get_ref(), board.org_id, auth.user_id).await?;
    }

    post_service::delete_post(pool.get_ref(), post_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn update_status(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateStatusRequest>,
) -> Result<HttpResponse, AppError> {
    let (_board_id, post_id) = path.into_inner();

    // Only org admin can change status
    let post = post_service::get_post_raw(pool.get_ref(), post_id).await?;
    let board = board_service::get_board_by_id(pool.get_ref(), post.board_id).await?;
    org_service::require_org_admin(pool.get_ref(), board.org_id, auth.user_id).await?;

    let updated = post_service::update_status(pool.get_ref(), post_id, &body.status).await?;
    let detail = post_service::get_post(pool.get_ref(), updated.id, auth.user_id).await?;
    let tags = tag_service::get_post_tags(pool.get_ref(), updated.id).await?;

    Ok(HttpResponse::Ok().json(build_detail_response(detail, tags)))
}

fn build_detail_response(
    row: crate::models::post::PostDetailRow,
    tags: Vec<crate::models::tag::Tag>,
) -> PostDetailResponse {
    let author = row.author_id.map(|id| PostAuthor {
        id,
        name: row.author_name.unwrap_or_default(),
        avatar_url: row.author_avatar_url,
    });

    PostDetailResponse {
        id: row.id,
        board_id: row.board_id,
        title: row.title,
        description: row.description,
        status: row.status.unwrap_or_else(|| "open".to_string()),
        vote_count: row.vote_count.unwrap_or(0),
        comment_count: row.comment_count.unwrap_or(0),
        pinned: row.pinned.unwrap_or(false),
        author,
        has_voted: row.has_voted,
        tags: tags.into_iter().map(TagResponse::from).collect(),
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}
