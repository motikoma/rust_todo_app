use chrono::prelude::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoVo {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl TodoVo {
    pub fn new(
        id: Uuid, title: String, description: String, created_at: DateTime<Utc>, updated_at: DateTime<Utc>
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            text: description,
            created_at: created_at,
            updated_at: updated_at
        }
    }

    pub fn reconstruct(
        id: Uuid, title: String, description: String,
        created_at: DateTime<Utc>, updated_at: DateTime<Utc>
    ) -> Self {
        Self {
            id,
            title,
            text: description,
            created_at,
            updated_at
        }
    }
}