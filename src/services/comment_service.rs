use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::comment::{Comment, CommentWithAuthorRow};

pub async fn get_comment_by_id(pool: &PgPool, comment_id: Uuid) -> Result<Comment, AppError> {
    sqlx::query_as("SELECT * FROM comments WHERE id = $1")
        .bind(comment_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))
}

pub async fn create_comment(
    pool: &PgPool,
    post_id: Uuid,
    author_id: Uuid,
    body: &str,
    is_admin_reply: bool,
) -> Result<Comment, AppError> {
    let mut tx = pool.begin().await?;

    let comment: Comment = sqlx::query_as(
        r#"
        INSERT INTO comments (post_id, author_id, body, is_admin_reply)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(post_id)
    .bind(author_id)
    .bind(body)
    .bind(is_admin_reply)
    .fetch_one(&mut *tx)
    .await?;

    // Increment comment count on the post
    sqlx::query("UPDATE posts SET comment_count = comment_count + 1 WHERE id = $1")
        .bind(post_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(comment)
}

pub async fn get_comments(
    pool: &PgPool,
    post_id: Uuid,
) -> Result<Vec<CommentWithAuthorRow>, AppError> {
    let comments = sqlx::query_as(
        r#"
        SELECT c.id, c.body, c.is_admin_reply, c.created_at,
               c.author_id, u.name as author_name, u.avatar_url as author_avatar_url
        FROM comments c
        LEFT JOIN users u ON u.id = c.author_id
        WHERE c.post_id = $1
        ORDER BY c.created_at ASC
        "#,
    )
    .bind(post_id)
    .fetch_all(pool)
    .await?;

    Ok(comments)
}

pub async fn delete_comment(
    pool: &PgPool,
    comment_id: Uuid,
    user_id: Uuid,
    is_admin: bool,
    post_id: Uuid,
) -> Result<(), AppError> {
    let mut tx = pool.begin().await?;

    // Atomic delete with ownership check via DELETE ... RETURNING
    let deleted: Option<Comment> = if is_admin {
        sqlx::query_as("DELETE FROM comments WHERE id = $1 RETURNING *")
            .bind(comment_id)
            .fetch_optional(&mut *tx)
            .await?
    } else {
        sqlx::query_as("DELETE FROM comments WHERE id = $1 AND author_id = $2 RETURNING *")
            .bind(comment_id)
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await?
    };

    if deleted.is_none() {
        return if is_admin {
            Err(AppError::NotFound("Comment not found".to_string()))
        } else {
            Err(AppError::Forbidden(
                "You can only delete your own comments".to_string(),
            ))
        };
    }

    // Decrement comment count
    sqlx::query("UPDATE posts SET comment_count = GREATEST(comment_count - 1, 0) WHERE id = $1")
        .bind(post_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}
