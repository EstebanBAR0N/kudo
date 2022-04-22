use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    active: bool,
}

impl User {
    pub fn new(id: i32, email: String, username: String) -> Self {
        User {
            id,
            email,
            username,
            active: true,
        }
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
