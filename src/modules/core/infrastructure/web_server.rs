
// src/modules/core/infrastructure/web_server.rs
use actix_web::{web, App, HttpServer, Responder, HttpResponse};

pub async fn start_server(server_port: i16) -> std::io::Result<()> {

    let server_address = format!("127.0.0.1:{}", server_port);

    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet))
    })
    .bind(server_address)?
    .run()
    .await
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// src/modules/core/infrastructure/repository.rs
// Implementations of repositories that interact with databases
