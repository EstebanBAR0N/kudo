use actix_web::{web, App, HttpServer};
use users_lib::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::routes()))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
