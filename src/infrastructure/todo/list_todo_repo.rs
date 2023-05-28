use async_trait::async_trait;
use sea_orm::{EntityTrait, DbConn};
use uuid::Uuid;
use anyhow::{Result, Ok};

use crate::{domain::todo::{repository::ListTodoRepo, todo::TodoVo}, entities::{self}};
use entities::{prelude::*};

pub struct ListTodoRepository {}

#[async_trait]
impl ListTodoRepo for ListTodoRepository {
    async fn list(&self, db: &DbConn) -> Result<Vec<TodoVo>> {
        let todos = Todo::find().all(db).await?;
        let todoVos = todos.iter().map(|todo|{
            TodoVo {                    
                id: Uuid::parse_str(&todo.id.to_string()).unwrap(),
                title: todo.title.clone(),
                text: todo.text.clone(),
                created_at: chrono::DateTime::from_utc(todo.created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_utc(todo.updated_at, chrono::Utc),
            }     
        }).collect();

        Ok(todoVos)
    }
}