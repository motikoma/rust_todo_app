use getset::Getters;
use sea_orm::DbConn;
use uuid::Uuid;
use anyhow::{anyhow, Result};

use crate::domain::todo::{repository::GetTodoRepo, todo::TodoVo};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct GetTodoUsecase;

impl GetTodoUsecase {
    pub async fn handle<T: GetTodoRepo>(&self, db: &DbConn, repository: &T, id: &str) -> Result<Option<TodoVo>> {
        let id = Uuid::parse_str(id)?;
        let todo = repository.get_todo_by_id(db, &id).await?;

        match todo {
            Some(todo) => Ok(Some(todo)),
            None => Err(anyhow!("Not found todo"))
        }
    }
}