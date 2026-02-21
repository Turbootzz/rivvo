use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::services::org_service;

pub async fn list_user_orgs(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let orgs = org_service::get_user_orgs(pool.get_ref(), auth.user_id).await?;
    Ok(HttpResponse::Ok().json(orgs))
}
