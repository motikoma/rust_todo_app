use async_trait::async_trait;
use sea_orm::{EntityTrait, DbConn};
use uuid::Uuid;
use anyhow::{Result, Ok};

use crate::{domain::todo::{repository::GetTodoRepo, todo::TodoVo}, entities};
use entities::{prelude::*, *};

pub struct GetTodoRepository {}

#[async_trait]
impl GetTodoRepo for GetTodoRepository {
    async fn get(&self, db: &DbConn, id: &Uuid) -> Result<Option<TodoVo>> {
        let todo: Option<todo::Model> = Todo::find_by_id(id.to_string()).one(db).await?;

        let todo = match todo {
            Some(todo) => {
                let todo = TodoVo {                    
                    id: Uuid::parse_str(&todo.id.to_string())?,
                    title: todo.title,
                    text: todo.text,
                    created_at: chrono::DateTime::from_utc(todo.created_at, chrono::Utc),
                    updated_at: chrono::DateTime::from_utc(todo.updated_at, chrono::Utc),
                };
                Some(todo)
            }
            None => None
        };
        Ok(todo)
    }
}