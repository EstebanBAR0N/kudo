use crate::users::User;
use actix_web::web::Path;
use actix_web::{get, HttpResponse};

enum UserStatus {
    NotFound(i32),
    Found(User),
}

#[get("/users/{id}")]
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
