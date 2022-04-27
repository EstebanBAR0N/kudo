use actix_web::{web, App, HttpResponse, HttpServer};
use controller_lib::workload;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(workload::routes::get_service())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
