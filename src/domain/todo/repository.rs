use async_trait::async_trait;
use sea_orm::DbConn;
use uuid::Uuid;
use anyhow::Result;
use super::todo::TodoVo;

#[async_trait]
pub trait CreateTodoRepo {
    async fn create(&self, db: &DbConn, todoVo: &TodoVo) -> Result<Uuid>;
}

#[async_trait]
pub trait ListTodoRepo {
    async fn list(&self, db: &DbConn) -> Result<Vec<TodoVo>>;
}

#[async_trait]
pub trait GetTodoRepo {
    async fn get(&self, db: &DbConn, id: &Uuid) -> Result<Option<TodoVo>>;
}

#[async_trait]
pub trait UpdateTodoRepo {
    async fn update(&self, db: &DbConn, id: &String, todoVo: &TodoVo) -> Result<TodoVo>;
}

#[async_trait]
pub trait DeleteTodoRepo {
    async fn delete(&self, db: &DbConn, id: &Uuid) -> Result<u64>;
}
