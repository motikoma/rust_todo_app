use async_trait::async_trait;
use sea_orm::{DbConn, EntityTrait};
use anyhow::{Result, Ok};
use uuid::Uuid;

use crate::{domain::todo::{repository::UpdateTodoRepo, todo::TodoVo}, entities::{todo, self}};
use entities::{prelude::*, *};

pub struct UpdateTodoRepository {}

#[async_trait]
impl UpdateTodoRepo for UpdateTodoRepository {
    async fn update(&self, db: &DbConn, id: &String, todo_vo: &TodoVo) -> Result<TodoVo> {
        
        let finded_todo: Option<todo::Model> = Todo::find_by_id(id.to_string()).one(db).await?;

        let mut finded_todo: todo::ActiveModel = finded_todo.unwrap().into();

        finded_todo.title = sea_orm::ActiveValue::Set(todo_vo.title.clone());
        finded_todo.text = sea_orm::ActiveValue::Set(todo_vo.text.clone());
        finded_todo.updated_at = sea_orm::ActiveValue::Set(todo_vo.updated_at.naive_utc());

        let updated_todo = Todo::update(finded_todo).exec(db).await?;
        println!("updated_todo: {:?}", updated_todo);

        let updated_todo_vo = TodoVo {
            id: Uuid::parse_str(&updated_todo.id.to_string())?,
            title: updated_todo.title,
            text: updated_todo.text,
            created_at: chrono::DateTime::from_utc(updated_todo.created_at, chrono::Utc),
            updated_at: chrono::DateTime::from_utc(updated_todo.updated_at, chrono::Utc),
        };

        Ok(updated_todo_vo)
    }
}