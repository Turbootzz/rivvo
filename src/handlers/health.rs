use actix_web::{HttpResponse, web};
use serde_json::json;
use sqlx::PgPool;

pub async fn health_check(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "ok",
            "database": "connected"
        })),
        Err(e) => {
            tracing::warn!("Health check failed: database unreachable: {e}");
            HttpResponse::ServiceUnavailable().json(json!({
                "status": "error",
                "database": "disconnected"
            }))
        }
    }
}
