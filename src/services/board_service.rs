use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::board::{Board, BoardWithCount};
use crate::utils::slugify::create_slug;

pub async fn create_board(
    pool: &PgPool,
    org_id: Uuid,
    name: &str,
    description: Option<&str>,
) -> Result<Board, AppError> {
    let slug = create_slug(name);

    sqlx::query_as(
        "INSERT INTO boards (org_id, name, slug, description) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(org_id)
    .bind(name)
    .bind(&slug)
    .bind(description)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            AppError::BadRequest(
                "A board with this name already exists in this organization".to_string(),
            )
        }
        _ => AppError::DatabaseError(e),
    })
}

pub async fn get_boards(pool: &PgPool, org_id: Uuid) -> Result<Vec<BoardWithCount>, AppError> {
    let boards = sqlx::query_as(
        r#"
        SELECT b.id, b.name, b.slug, b.description,
               COUNT(p.id)::bigint as post_count
        FROM boards b
        LEFT JOIN posts p ON p.board_id = b.id
        WHERE b.org_id = $1
        GROUP BY b.id
        ORDER BY b.created_at ASC
        "#,
    )
    .bind(org_id)
    .fetch_all(pool)
    .await?;

    Ok(boards)
}

pub async fn get_board_by_slug(pool: &PgPool, org_id: Uuid, slug: &str) -> Result<Board, AppError> {
    sqlx::query_as("SELECT * FROM boards WHERE org_id = $1 AND slug = $2")
        .bind(org_id)
        .bind(slug)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Board not found".to_string()))
}

pub async fn get_board_by_id(pool: &PgPool, board_id: Uuid) -> Result<Board, AppError> {
    sqlx::query_as("SELECT * FROM boards WHERE id = $1")
        .bind(board_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Board not found".to_string()))
}

pub async fn update_board(
    pool: &PgPool,
    board_id: Uuid,
    name: &str,
    description: Option<&str>,
) -> Result<Board, AppError> {
    let slug = create_slug(name);

    sqlx::query_as(
        "UPDATE boards SET name = $1, slug = $2, description = $3 WHERE id = $4 RETURNING *",
    )
    .bind(name)
    .bind(&slug)
    .bind(description)
    .bind(board_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            AppError::BadRequest(
                "A board with this name already exists in this organization".to_string(),
            )
        }
        _ => AppError::DatabaseError(e),
    })
}

pub async fn delete_board(pool: &PgPool, board_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM boards WHERE id = $1")
        .bind(board_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Board not found".to_string()));
    }
    Ok(())
}
