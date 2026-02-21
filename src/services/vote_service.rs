use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::vote::{Vote, VoteResult};

pub async fn toggle_vote(
    pool: &PgPool,
    post_id: Uuid,
    user_id: Uuid,
) -> Result<VoteResult, AppError> {
    // Check if vote already exists
    let existing: Option<Vote> =
        sqlx::query_as("SELECT * FROM votes WHERE post_id = $1 AND user_id = $2")
            .bind(post_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

    if existing.is_some() {
        // Remove vote and decrement count
        sqlx::query("DELETE FROM votes WHERE post_id = $1 AND user_id = $2")
            .bind(post_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        let row: (i32,) = sqlx::query_as(
            "UPDATE posts SET vote_count = GREATEST(vote_count - 1, 0) WHERE id = $1 RETURNING vote_count",
        )
        .bind(post_id)
        .fetch_one(pool)
        .await?;

        Ok(VoteResult {
            voted: false,
            vote_count: row.0,
        })
    } else {
        // Add vote and increment count
        sqlx::query("INSERT INTO votes (post_id, user_id) VALUES ($1, $2)")
            .bind(post_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        let row: (i32,) = sqlx::query_as(
            "UPDATE posts SET vote_count = vote_count + 1 WHERE id = $1 RETURNING vote_count",
        )
        .bind(post_id)
        .fetch_one(pool)
        .await?;

        Ok(VoteResult {
            voted: true,
            vote_count: row.0,
        })
    }
}
