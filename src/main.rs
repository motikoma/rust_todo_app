use std::env;

use actix_web::{App, HttpServer};
use actix_web::{get, HttpResponse, Responder, middleware, Result, web};
use serde::Serialize;

use sea_orm::{Database, DbErr, ConnectionTrait, DbBackend, Statement, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use listenfd::ListenFd;

mod api;
mod domain;
mod infrastructure;
mod usecase;
mod entities;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // establish connection to database and apply migrations
    // -> create table if not exists
    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    // build app state
    let state = AppState{ conn };

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(api::api::config)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?
    };

    println!("Server running at http://{}/", &server_url);
    server.run().await?;

    Ok(())
}