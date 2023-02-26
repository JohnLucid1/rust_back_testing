// mod structure;
// use structure::Data;

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let cards: Data = reqwest::get("http://127.0.0.1:8090/api/collections/card/records")
//         .await?
//         .json()
//         .await?;

//     println!("{:#?}", cards);
//     Ok(())
// }

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello)))

    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}