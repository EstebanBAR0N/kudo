use crate::users::User;
use actix_web::dev::{HttpServiceFactory, ServiceFactory};
use actix_web::web::{self, Path};
use actix_web::{get, services, HttpResponse, Scope};

pub fn routes() -> Scope {
    let service = web::scope("/user")
        .service(get_user)
        .service(get_users)
        .service(get_test);
    return service;
}

enum UserStatus {
    NotFound(i32),
    Found(User),
}

#[get("/test")]
pub async fn get_test() -> HttpResponse {
    HttpResponse::Ok().body("Hello test")
}

#[get("/{id}")]
pub async fn get_user(path: Path<i32>) -> HttpResponse {
    let id = path.into_inner();

    // on va chercher en base de données l'id et on retourne un status
    // pour l'exemple on imagine qu'on a trouvé un utilisateur que j'ai crée
    let user_example = User::new(id, "max@gmail.com".to_string(), "max".to_string());

    let status: UserStatus = UserStatus::Found(user_example);

    match status {
        UserStatus::Found(user) => HttpResponse::Ok().json(user),

        UserStatus::NotFound(id) => HttpResponse::NotFound().body(format!("Not found : {}", id)),
    }
}

#[get("/")]
pub async fn get_users() -> HttpResponse {
    HttpResponse::Ok().body("Hello world")
}
