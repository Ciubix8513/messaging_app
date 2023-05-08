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

#[derive(Serialize, Deserialize)]
pub struct ChangePassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendInvite {
    pub chat_id: i32,
    pub recipient_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GetInvites {
    pub invite_id: i32,
    pub chat_id: i32,
    pub chat_name: String,
    pub sender_name: String,
    pub created_at: chrono::NaiveDateTime,
}
