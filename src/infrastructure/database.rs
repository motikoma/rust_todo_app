use std::fmt::Error;
use chrono::prelude::*;
use uuid::Uuid;
use std::sync::{Arc, Mutex};

use crate::domain::todo::todo::TodoVo;

// 一時的にメモリ上にデータを保持するパターン
pub struct Database {
    pub todos: Arc<Mutex<Vec<TodoVo>>>,
}

impl Database {
    pub fn new() -> Self{
        let todos = Arc::new(Mutex::new(vec![]));
        Self { todos }
    }

    pub fn create_todo(&self, todo: TodoVo) -> Result<TodoVo, Error> {
        let mut todos = self.todos.lock().unwrap();
        let id = uuid::Uuid::new_v4();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let todo = TodoVo {
            id: Some(id),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            ..todo
        };
        todos.push(todo.clone());
        Ok(todo)
    }

    pub fn get_todos(&self) -> Vec<TodoVo> {
        let todos = self.todos.lock().unwrap();
        todos.clone()
    }

    pub fn get_todo_by_id(&self, id: &str) -> Option<TodoVo> {
        let todos = self.todos.lock().unwrap();
        todos.iter().find(|todo| todo.id == Some(
            Uuid::parse_str(id).unwrap()
        )).cloned()
    }

    pub fn update_todo_by_id(&self, id: &str, todo: TodoVo) -> Option<TodoVo>{
        let mut todos = self.todos.lock().unwrap();
        let updated_at = Utc::now();
        let todo = TodoVo{
            id: Some(            Uuid::parse_str(id).unwrap()),
            updated_at: Some(updated_at),
            ..todo
        };
        let index = todos.iter().position(|todo| todo.id == Some(Uuid::parse_str(id).unwrap()))?;
        todos[index] = todo.clone();
        Some(todo)
    }

    pub fn delete_todo_by_id(&self, id: &str) -> Option<TodoVo>{
        let mut todos = self.todos.lock().unwrap();
        let index = todos.iter().position(|todo| todo.id == Some(Uuid::parse_str(id).unwrap()))?;
        Some(todos.remove(index))
    }
}