use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub username: String,
    pub user_id: i32,
}
