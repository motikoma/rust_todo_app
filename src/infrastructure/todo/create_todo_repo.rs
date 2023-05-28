use async_trait::async_trait;
use sea_orm::{DbConn, EntityTrait};
use anyhow::{Result, Ok};
use uuid::Uuid;

use crate::{domain::todo::{repository::CreateTodoRepo, todo::TodoVo}, entities::{todo, self}};
use entities::{prelude::*, *};

pub struct CreateTodoRepository {}

#[async_trait]
impl CreateTodoRepo for CreateTodoRepository {
    async fn create(&self, db: &DbConn, createTodo: &TodoVo) -> Result<Uuid> {

        let new_todo = todo::ActiveModel {
            id: sea_orm::ActiveValue::Set(createTodo.id.to_string()),
            title: sea_orm::ActiveValue::Set(createTodo.title.clone()),
            text: sea_orm::ActiveValue::Set(createTodo.text.clone()),
            created_at: sea_orm::ActiveValue::Set(createTodo.created_at.naive_utc()),
            updated_at: sea_orm::ActiveValue::Set(createTodo.updated_at.naive_utc()),
        };

        let inserted_todo = Todo::insert(new_todo).exec(db).await?;
        let inserted_id = Uuid::parse_str(&inserted_todo.last_insert_id)?;

        Ok(inserted_id)
    }
}