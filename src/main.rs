use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Bu mening birinchi web sayitim!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}