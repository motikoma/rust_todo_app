use getset::Getters;
use sea_orm::DbConn;
use anyhow::{Result};

use crate::domain::todo::{repository::UpdateTodoRepo, todo::TodoVo};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct UpdateTodoUsecase;

impl UpdateTodoUsecase {
    pub async fn handle(&self, db: &DbConn, repository: &impl UpdateTodoRepo, id: &String, todo: &TodoVo) -> Result<TodoVo> {
        let updated_todo = repository.update(db, id, todo).await?;
        Ok(updated_todo)
    }
}