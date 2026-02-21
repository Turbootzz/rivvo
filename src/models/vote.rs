use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Vote {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct VoteResult {
    pub voted: bool,
    pub vote_count: i32,
}
