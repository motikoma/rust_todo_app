use async_trait::async_trait;
use sea_orm::DbConn;
use uuid::Uuid;
use anyhow::Result;
use super::todo::TodoVo;

#[async_trait]
pub trait CreateTodoRepo {
    async fn create(&self, title: &Uuid) -> Result<TodoVo>;
}

#[async_trait]
pub trait ListTodoRepo {
    async fn list() -> Result<Vec<TodoVo>>;
}

#[async_trait]
pub trait GetTodoRepo {
    async fn get_todo_by_id(&self, db: &DbConn ,id: &Uuid) -> Result<Option<TodoVo>>;
}

#[async_trait]
pub trait UpdateTodoRepo {
    async fn update(id: &Uuid) -> Result<TodoVo>;
}

#[async_trait]
pub trait DeleteTodoRepo {
    async fn delete(id: &Uuid) -> Result<()>;
}
