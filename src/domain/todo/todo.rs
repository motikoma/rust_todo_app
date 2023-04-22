use chrono::prelude::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn new(
        id: Option<String>, title: String, description: Option<String>
    ) -> Self {
        Self {
            id,title,description,created_at: None,updated_at: None
        }
    }
}