use actix_web::{App, HttpServer};
use users_lib::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::get_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
