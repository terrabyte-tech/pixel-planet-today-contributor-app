
use actix_web::{HttpServer, App, web, HttpResponse};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::to(|| {
                HttpResponse::Ok().body("Hello, World!")
            }))
    })
    .bind("127.0.0.1:8001")?
        .run()
        .await
}
