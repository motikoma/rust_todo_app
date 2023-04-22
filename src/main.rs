use actix_web::{get, HttpResponse, Responder, Result};
use serde::Serialize;

mod api;
mod domain;
mod infrastructure;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "ok".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
            .default_service(actix_web::web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}