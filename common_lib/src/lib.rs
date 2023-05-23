use serde_derive::{Deserialize, Serialize};

pub mod encryption;
pub mod grimoire;

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
    pub private_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangePassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendInvite {
    pub chat_id: i32,
    pub recipient_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetInvites {
    pub invite_id: i32,
    pub chat_id: i32,
    pub chat_name: String,
    pub sender_name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct SendMessage {
    pub chat_id: i32,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetMessage {
    pub message_id: i32,
    pub user_id: i32,
    pub username: String,
    pub message_text: String,
    pub sent_at: chrono::NaiveDateTime,
    pub files: Vec<GetFile>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetFile {
    pub file_id: i32,
    pub filename: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetChat {
    pub chat_id: i32,
    pub chat_name: String,
    pub creator_id: i32,
    pub creator_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadFile {
    pub chat_id: i32,
    pub message_text: String,
}
