#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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