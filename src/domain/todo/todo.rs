use chrono::prelude::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoVo {
    pub id: Option<Uuid>,
    pub title: String,
    pub text: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TodoVo {
    pub fn new(
        id: Option<Uuid>, title: String, description: Option<String>
    ) -> Self {
        Self {
            id,
            title,
            text: description,
            created_at: None,
            updated_at: None
        }
    }

    pub fn reconstruct(
        id: Option<Uuid>, title: String, description: Option<String>,
        created_at: Option<DateTime<Utc>>, updated_at: Option<DateTime<Utc>>
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