use serde::{Deserialize, Serialize};


#[derive(Debug,Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}
pub struct UserService {
    // Dependencies, e.g., a database connection pool
}

impl UserService {
    pub fn new() -> Self {
        UserService {}
    }

    pub async fn list_users(&self) -> Vec<User> {
        vec![
            User { id: 1, name: "Alice".to_string() },
            User { id: 2, name: "Bob".to_string() },
        ]
    }

    pub async fn create_user(&self) -> User {
        User { id: 3, name: "Carol".to_string() }
    }
}