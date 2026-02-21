use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::comment::{CommentAuthor, CommentResponse};
use crate::services::{board_service, comment_service, org_service, post_service};

#[derive(Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(
        min = 1,
        max = 10000,
        message = "Comment must be between 1 and 10000 characters"
    ))]
    pub body: String,
}

pub async fn list_comments(
    pool: web::Data<PgPool>,
    _auth: AuthenticatedUser,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = post_id.into_inner();
    let rows = comment_service::get_comments(pool.get_ref(), post_id).await?;

    let response: Vec<CommentResponse> = rows
        .into_iter()
        .map(|row| CommentResponse {
            id: row.id,
            body: row.body,
            is_admin_reply: row.is_admin_reply.unwrap_or(false),
            author: row.author_id.map(|id| CommentAuthor {
                id,
                name: row.author_name.unwrap_or_default(),
                avatar_url: row.author_avatar_url,
            }),
            created_at: row.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_comment(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    post_id: web::Path<Uuid>,
    body: web::Json<CreateCommentRequest>,
) -> Result<HttpResponse, AppError> {
    let post_id = post_id.into_inner();
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Check if user is admin to mark as admin reply
    let post = post_service::get_post_raw(pool.get_ref(), post_id).await?;
    let board = board_service::get_board_by_id(pool.get_ref(), post.board_id).await?;
    let is_admin = org_service::is_org_admin(pool.get_ref(), board.org_id, auth.user_id)
        .await
        .unwrap_or(false);

    let comment = comment_service::create_comment(
        pool.get_ref(),
        post_id,
        auth.user_id,
        &body.body,
        is_admin,
    )
    .await?;

    Ok(HttpResponse::Created().json(CommentResponse {
        id: comment.id,
        body: comment.body,
        is_admin_reply: comment.is_admin_reply.unwrap_or(false),
        author: Some(CommentAuthor {
            id: auth.user_id,
            name: String::new(), // Will be filled by frontend from auth state
            avatar_url: None,
        }),
        created_at: comment.created_at,
    }))
}

pub async fn delete_comment(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    comment_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let comment_id = comment_id.into_inner();
    comment_service::delete_comment(pool.get_ref(), comment_id, auth.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
