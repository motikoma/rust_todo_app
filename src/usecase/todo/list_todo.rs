use getset::Getters;
use sea_orm::DbConn;
use anyhow::{Result};

use crate::domain::todo::{repository::{ListTodoRepo}, todo::TodoVo};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct ListTodoUsecase;

impl ListTodoUsecase {
    pub async fn handle(&self, db: &DbConn, repository: &impl ListTodoRepo) -> Result<Vec<TodoVo>> {
        Ok(repository.list(db).await?)
    }
}