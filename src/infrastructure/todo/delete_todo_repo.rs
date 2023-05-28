use async_trait::async_trait;
use sea_orm::{DbConn, EntityTrait};
use anyhow::{Result, Ok};
use uuid::Uuid;

use crate::{domain::todo::{repository::DeleteTodoRepo}, entities::{self}};
use entities::{prelude::*};

pub struct DeleteTodoRepository {}

#[async_trait]
impl DeleteTodoRepo for DeleteTodoRepository {
    async fn delete(&self, db: &DbConn, id: &Uuid) -> Result<u64> {

        let deleted_todo = Todo::delete_by_id(id.to_string()).exec(db).await?;

        Ok(deleted_todo.rows_affected)
    }
}