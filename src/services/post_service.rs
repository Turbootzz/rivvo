use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::post::{Post, PostDetailRow, PostListRow, VALID_STATUSES};

pub async fn create_post(
    pool: &PgPool,
    board_id: Uuid,
    author_id: Uuid,
    title: &str,
    description: Option<&str>,
) -> Result<Post, AppError> {
    sqlx::query_as(
        r#"
        INSERT INTO posts (board_id, author_id, title, description, status)
        VALUES ($1, $2, $3, $4, 'open')
        RETURNING *
        "#,
    )
    .bind(board_id)
    .bind(author_id)
    .bind(title)
    .bind(description)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)
}

pub async fn get_posts(
    pool: &PgPool,
    board_id: Uuid,
    user_id: Uuid,
    sort: &str,
    status_filter: Option<&str>,
) -> Result<Vec<PostListRow>, AppError> {
    let order_clause = match sort {
        "recent" => "p.created_at DESC",
        "oldest" => "p.created_at ASC",
        _ => "p.vote_count DESC, p.created_at DESC", // "votes" default
    };

    // Build query dynamically based on filter
    let query = if status_filter.is_some() {
        format!(
            r#"
            SELECT p.id, p.title, p.description, p.status, p.vote_count, p.comment_count,
                   p.pinned, u.name as author_name, p.created_at,
                   EXISTS(SELECT 1 FROM votes v WHERE v.post_id = p.id AND v.user_id = $2) as has_voted
            FROM posts p
            LEFT JOIN users u ON u.id = p.author_id
            WHERE p.board_id = $1 AND p.status = $3
            ORDER BY p.pinned DESC, {order_clause}
            "#
        )
    } else {
        format!(
            r#"
            SELECT p.id, p.title, p.description, p.status, p.vote_count, p.comment_count,
                   p.pinned, u.name as author_name, p.created_at,
                   EXISTS(SELECT 1 FROM votes v WHERE v.post_id = p.id AND v.user_id = $2) as has_voted
            FROM posts p
            LEFT JOIN users u ON u.id = p.author_id
            WHERE p.board_id = $1
            ORDER BY p.pinned DESC, {order_clause}
            "#
        )
    };

    let rows = if let Some(status) = status_filter {
        sqlx::query_as::<_, PostListRow>(&query)
            .bind(board_id)
            .bind(user_id)
            .bind(status)
            .fetch_all(pool)
            .await?
    } else {
        sqlx::query_as::<_, PostListRow>(&query)
            .bind(board_id)
            .bind(user_id)
            .fetch_all(pool)
            .await?
    };

    Ok(rows)
}

pub async fn get_post(
    pool: &PgPool,
    post_id: Uuid,
    user_id: Uuid,
) -> Result<PostDetailRow, AppError> {
    sqlx::query_as(
        r#"
        SELECT p.id, p.board_id, p.title, p.description, p.status, p.vote_count, p.comment_count,
               p.pinned, p.created_at, p.updated_at,
               p.author_id, u.name as author_name, u.email as author_email, u.avatar_url as author_avatar_url,
               EXISTS(SELECT 1 FROM votes v WHERE v.post_id = p.id AND v.user_id = $2) as has_voted
        FROM posts p
        LEFT JOIN users u ON u.id = p.author_id
        WHERE p.id = $1
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Post not found".to_string()))
}

pub async fn get_post_raw(pool: &PgPool, post_id: Uuid) -> Result<Post, AppError> {
    sqlx::query_as("SELECT * FROM posts WHERE id = $1")
        .bind(post_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))
}

pub async fn update_post(
    pool: &PgPool,
    post_id: Uuid,
    title: &str,
    description: Option<&str>,
) -> Result<Post, AppError> {
    sqlx::query_as(
        "UPDATE posts SET title = $1, description = $2, updated_at = now() WHERE id = $3 RETURNING *",
    )
    .bind(title)
    .bind(description)
    .bind(post_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)
}

pub async fn delete_post(pool: &PgPool, post_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(post_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Post not found".to_string()));
    }
    Ok(())
}

pub async fn update_status(pool: &PgPool, post_id: Uuid, status: &str) -> Result<Post, AppError> {
    if !VALID_STATUSES.contains(&status) {
        return Err(AppError::BadRequest(format!(
            "Invalid status. Must be one of: {}",
            VALID_STATUSES.join(", ")
        )));
    }

    sqlx::query_as("UPDATE posts SET status = $1, updated_at = now() WHERE id = $2 RETURNING *")
        .bind(status)
        .bind(post_id)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)
}
