use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub board_id: Uuid,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TagResponse {
    pub id: Uuid,
    pub name: String,
    pub color: String,
}

impl From<Tag> for TagResponse {
    fn from(t: Tag) -> Self {
        TagResponse {
            id: t.id,
            name: t.name,
            color: t.color.unwrap_or_else(|| "#6366f1".to_string()),
        }
    }
}
