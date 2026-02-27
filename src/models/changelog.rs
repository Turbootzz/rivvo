use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct ChangelogEntry {
    pub id: Uuid,
    pub org_id: Uuid,
    pub title: String,
    pub body: String,
    pub published_at: Option<DateTime<Utc>>,
    pub is_draft: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ChangelogResponse {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published_at: Option<DateTime<Utc>>,
    pub is_draft: bool,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<ChangelogEntry> for ChangelogResponse {
    fn from(e: ChangelogEntry) -> Self {
        Self {
            id: e.id,
            title: e.title,
            body: e.body,
            published_at: e.published_at,
            is_draft: e.is_draft.unwrap_or(true),
            created_at: e.created_at,
        }
    }
}
