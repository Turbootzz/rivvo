use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::changelog::ChangelogEntry;

pub async fn get_published(pool: &PgPool, org_id: Uuid) -> Result<Vec<ChangelogEntry>, AppError> {
    sqlx::query_as(
        "SELECT * FROM changelog_entries WHERE org_id = $1 AND is_draft = FALSE ORDER BY published_at DESC",
    )
    .bind(org_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)
}

pub async fn get_all(pool: &PgPool, org_id: Uuid) -> Result<Vec<ChangelogEntry>, AppError> {
    sqlx::query_as("SELECT * FROM changelog_entries WHERE org_id = $1 ORDER BY created_at DESC")
        .bind(org_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<ChangelogEntry, AppError> {
    sqlx::query_as("SELECT * FROM changelog_entries WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Changelog entry not found".to_string()))
}

pub async fn create(
    pool: &PgPool,
    org_id: Uuid,
    title: &str,
    body: &str,
) -> Result<ChangelogEntry, AppError> {
    sqlx::query_as(
        "INSERT INTO changelog_entries (org_id, title, body) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(org_id)
    .bind(title)
    .bind(body)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    title: &str,
    body: &str,
    is_draft: bool,
    published_at: Option<DateTime<Utc>>,
) -> Result<ChangelogEntry, AppError> {
    sqlx::query_as(
        r#"
        UPDATE changelog_entries
        SET title = $1, body = $2, is_draft = $3, published_at = $4, updated_at = now()
        WHERE id = $5
        RETURNING *
        "#,
    )
    .bind(title)
    .bind(body)
    .bind(is_draft)
    .bind(published_at)
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM changelog_entries WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Changelog entry not found".to_string()));
    }
    Ok(())
}
