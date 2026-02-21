use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::vote::{Vote, VoteResult};

pub async fn toggle_vote(
    pool: &PgPool,
    post_id: Uuid,
    user_id: Uuid,
) -> Result<VoteResult, AppError> {
    let mut tx = pool.begin().await?;

    // Lock the parent post row to serialize concurrent vote toggles
    sqlx::query("SELECT 1 FROM posts WHERE id = $1 FOR UPDATE")
        .bind(post_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    // Check if vote already exists
    let existing: Option<Vote> =
        sqlx::query_as("SELECT * FROM votes WHERE post_id = $1 AND user_id = $2")
            .bind(post_id)
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await?;

    if existing.is_some() {
        // Remove vote and decrement count
        sqlx::query("DELETE FROM votes WHERE post_id = $1 AND user_id = $2")
            .bind(post_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

        let row: (i32,) = sqlx::query_as(
            "UPDATE posts SET vote_count = GREATEST(vote_count - 1, 0) WHERE id = $1 RETURNING vote_count",
        )
        .bind(post_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(VoteResult {
            voted: false,
            vote_count: row.0,
        })
    } else {
        // Add vote and increment count
        sqlx::query("INSERT INTO votes (post_id, user_id) VALUES ($1, $2)")
            .bind(post_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

        let row: (i32,) = sqlx::query_as(
            "UPDATE posts SET vote_count = vote_count + 1 WHERE id = $1 RETURNING vote_count",
        )
        .bind(post_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(VoteResult {
            voted: true,
            vote_count: row.0,
        })
    }
}
