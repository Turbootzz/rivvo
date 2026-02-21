use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Board {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_private: Option<bool>,
    pub settings: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct BoardWithCount {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub post_count: i64,
}

#[derive(Debug, Serialize)]
pub struct BoardResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub post_count: i64,
}

impl From<BoardWithCount> for BoardResponse {
    fn from(b: BoardWithCount) -> Self {
        BoardResponse {
            id: b.id,
            name: b.name,
            slug: b.slug,
            description: b.description,
            post_count: b.post_count,
        }
    }
}
