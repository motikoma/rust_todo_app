use getset::Getters;
use sea_orm::DbConn;
use uuid::Uuid;
use anyhow::{anyhow, Result};

use crate::domain::todo::{repository::DeleteTodoRepo};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct DeleteTodoUsecase;

impl DeleteTodoUsecase {
    pub async fn handle(&self, db: &DbConn, repository: &impl DeleteTodoRepo, id: &Uuid) -> Result<u64> {
        let deleted_row = repository.delete(db, id).await?;
        Ok(deleted_row)
    }
}