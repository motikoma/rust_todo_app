use std::env;

use actix_web::{App, HttpServer};
use actix_web::{get, HttpResponse, Responder, Result, web};
use serde::Serialize;

use futures::executor::block_on;
use sea_orm::{Database, DbErr, ConnectionTrait, DbBackend, Statement, DatabaseConnection};

mod api;
mod domain;
mod infrastructure;
mod usecase;
mod entities;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection
}

async fn run() -> Result<(), DbErr> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let db = Database::connect(&db_url).await?;

    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", &db_name),
            ))
            .await?;

            let url = format!("{}/{}", &db_url, &db_name);
            Database::connect(&url).await?
        }
        _ => unimplemented!(),
    };
    
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let todo_db = infrastructure::database::Database::new();
    let app_data = web::Data::new(todo_db);

    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}