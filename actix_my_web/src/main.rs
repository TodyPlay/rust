use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_my_web::handler::controller;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(controller::index))
            .route("/again", web::get().to(controller::index2))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}