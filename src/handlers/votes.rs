use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::services::vote_service;

pub async fn toggle_vote(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = post_id.into_inner();
    let result = vote_service::toggle_vote(pool.get_ref(), post_id, auth.user_id).await?;
    Ok(HttpResponse::Ok().json(result))
}
