use getset::Getters;
use sea_orm::DbConn;
use uuid::Uuid;
use anyhow::{anyhow, Result};

use crate::domain::todo::{repository::CreateTodoRepo, todo::TodoVo};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct CreateTodoUsecase;

impl CreateTodoUsecase {
    pub async fn handle(&self, db: &DbConn, repository: &impl CreateTodoRepo, todo: &TodoVo) -> Result<Uuid> {
        let created_todo_id = repository.create(db, todo).await?;
        Ok(created_todo_id)
    }
}