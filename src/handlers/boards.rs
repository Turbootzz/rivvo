use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::board::BoardResponse;
use crate::services::{board_service, org_service};

#[derive(Deserialize, Validate)]
pub struct CreateBoardRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateBoardRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,
    pub description: Option<String>,
}

pub async fn list_boards(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    org_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let org_id = org_id.into_inner();
    org_service::get_member(pool.get_ref(), org_id, auth.user_id).await?;

    let boards = board_service::get_boards(pool.get_ref(), org_id).await?;
    let response: Vec<BoardResponse> = boards.into_iter().map(BoardResponse::from).collect();

    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_board(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    org_id: web::Path<Uuid>,
    body: web::Json<CreateBoardRequest>,
) -> Result<HttpResponse, AppError> {
    let org_id = org_id.into_inner();
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    org_service::require_org_admin(pool.get_ref(), org_id, auth.user_id).await?;

    let board = board_service::create_board(
        pool.get_ref(),
        org_id,
        &body.name,
        body.description.as_deref(),
    )
    .await?;

    Ok(HttpResponse::Created().json(BoardResponse {
        id: board.id,
        name: board.name,
        slug: board.slug,
        description: board.description,
        post_count: 0,
    }))
}

pub async fn get_board(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, String)>,
) -> Result<HttpResponse, AppError> {
    let (org_id, slug) = path.into_inner();
    org_service::get_member(pool.get_ref(), org_id, auth.user_id).await?;

    let board = board_service::get_board_by_slug(pool.get_ref(), org_id, &slug).await?;

    Ok(HttpResponse::Ok().json(BoardResponse {
        id: board.id,
        name: board.name,
        slug: board.slug,
        description: board.description,
        post_count: 0, // detail view doesn't need count
    }))
}

pub async fn update_board(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, String)>,
    body: web::Json<UpdateBoardRequest>,
) -> Result<HttpResponse, AppError> {
    let (org_id, slug) = path.into_inner();
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    org_service::require_org_admin(pool.get_ref(), org_id, auth.user_id).await?;

    let board = board_service::get_board_by_slug(pool.get_ref(), org_id, &slug).await?;
    let updated = board_service::update_board(
        pool.get_ref(),
        board.id,
        &body.name,
        body.description.as_deref(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(BoardResponse {
        id: updated.id,
        name: updated.name,
        slug: updated.slug,
        description: updated.description,
        post_count: 0,
    }))
}

pub async fn delete_board(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    path: web::Path<(Uuid, String)>,
) -> Result<HttpResponse, AppError> {
    let (org_id, slug) = path.into_inner();
    org_service::require_org_admin(pool.get_ref(), org_id, auth.user_id).await?;

    let board = board_service::get_board_by_slug(pool.get_ref(), org_id, &slug).await?;
    board_service::delete_board(pool.get_ref(), board.id).await?;

    Ok(HttpResponse::NoContent().finish())
}
