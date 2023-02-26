mod api;
mod repository;
use api::task:: {
    get_task
};


use actix_web::{get, HttpServer, middleware::Logger, App, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}