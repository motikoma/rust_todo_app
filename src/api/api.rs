use actix_web::*;
use actix_web::{web::{Data, Json}, post, HttpResponse};
use serde::Serialize;
use crate::{AppState, usecase};
use crate::domain::todo::todo::TodoVo;
use crate::infrastructure::database::Database;
use crate::infrastructure::todo::get_todo_repo::TodoRepository;

async fn not_found() -> Result<HttpResponse, actix_web::Error> {
    let responseBody = "not found";
    Ok(HttpResponse::NotFound().body(responseBody))
}

#[get("/health")]
async fn health() -> Result<HttpResponse, actix_web::Error> {
    let responseBody = "Ok";
    Ok(HttpResponse::Ok().body(responseBody))
}

#[post("/todos")]
pub async fn create_todo(db: Data<Database>, new_todo: Json<TodoVo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
pub async fn get_todos(db: Data<Database>) -> HttpResponse {
    let todos = db.get_todos();
    HttpResponse::Ok().json(todos)
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(db: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let conn = &db.conn;
    let repository = TodoRepository{};
    let todo = usecase::todo::get_todo::GetTodoUsecase{}.handle(conn, &repository, &id).await;

    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo_by_id(db: Data<Database>, id: web::Path<String>, updated_todo: web::Json<TodoVo>) -> HttpResponse {
    let todo = db.update_todo_by_id(&id, updated_todo.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let todo = db.delete_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .default_service(actix_web::web::route().to(not_found))
            .service(health)
            .service(create_todo)
            .service(get_todos)
            .service(get_todo_by_id)
            .service(update_todo_by_id)
            .service(delete_todo_by_id)
    );
}