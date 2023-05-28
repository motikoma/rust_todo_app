use actix_web::*;
use actix_web::{web::{Data, Json}, post, HttpResponse};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::infrastructure::todo::create_todo_repo::CreateTodoRepository;
use crate::infrastructure::todo::get_todo_repo::GetTodoRepository;
use crate::infrastructure::todo::list_todo_repo::ListTodoRepository;
use crate::infrastructure::todo::update_todo_repo::UpdateTodoRepository;
use crate::infrastructure::todo::delete_todo_repo::DeleteTodoRepository;
use crate::{AppState, usecase};
use crate::domain::todo::todo::TodoVo;

async fn not_found() -> Result<HttpResponse, actix_web::Error> {
    let responseBody = "not found";
    Ok(HttpResponse::NotFound().body(responseBody))
}

#[get("/health")]
async fn health() -> Result<HttpResponse, actix_web::Error> {
    let responseBody = "Ok";
    Ok(HttpResponse::Ok().body(responseBody))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateTodoRequestBody {
    title: String,
    description: String,
}

#[post("/todos")]
pub async fn create_todo(db: web::Data<AppState>, new_todo: Json<CreateTodoRequestBody>) -> HttpResponse {
    let conn = &db.conn;
    let repository = CreateTodoRepository{};
    let todo_vo = TodoVo::new(
        uuid::Uuid::new_v4(),
        new_todo.title.clone(),
        new_todo.description.clone(),
        chrono::Utc::now(),
        chrono::Utc::now()
    );

    let result = usecase::todo::create_todo::CreateTodoUsecase{}.handle(conn, &repository, &todo_vo).await;

    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
pub async fn get_todos(db: web::Data<AppState>) -> HttpResponse {
    let conn = &db.conn;
    let repository = ListTodoRepository{};
    let todos = usecase::todo::list_todo::ListTodoUsecase{}.handle(conn, &repository).await;

    match todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(db: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let conn = &db.conn;
    let repository = GetTodoRepository{};
    let todo = usecase::todo::get_todo::GetTodoUsecase{}.handle(conn, &repository, &id).await;

    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// TODO: updated_todoは一部のTodoVoの一部の情報のみを格納するので修正が必要
#[put("/todos/{id}")]
pub async fn update_todo_by_id(db: web::Data<AppState>, id: web::Path<String>, updated_todo: web::Json<TodoVo>) -> HttpResponse {
    let conn = &db.conn;
    let repository = UpdateTodoRepository{};
    let todo = usecase::todo::update_todo::UpdateTodoUsecase{}.handle(conn, &repository, &id, &updated_todo).await;

    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(db: Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let conn = &db.conn;
    let repository = DeleteTodoRepository{};
    let id = Uuid::parse_str(&id).unwrap();
    let todo = usecase::todo::delete_todo::DeleteTodoUsecase{}.handle(conn, &repository, &id).await;

    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
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