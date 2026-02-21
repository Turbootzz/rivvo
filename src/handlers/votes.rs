use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::services::{board_service, org_service, post_service, vote_service};

pub async fn toggle_vote(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = post_id.into_inner();

    // Verify user is org member
    let post = post_service::get_post_raw(pool.get_ref(), post_id).await?;
    let board = board_service::get_board_by_id(pool.get_ref(), post.board_id).await?;
    org_service::get_member(pool.get_ref(), board.org_id, auth.user_id).await?;

    let result = vote_service::toggle_vote(pool.get_ref(), post_id, auth.user_id).await?;
    Ok(HttpResponse::Ok().json(result))
}
