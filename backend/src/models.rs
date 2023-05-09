use crate::schema::*;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable)]
pub struct GroupChat {
    pub id: i32,
    pub name: String,
    pub created_by: i32,
    pub creation_date: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable)]
pub struct GroupChatMember {
    pub chat_id: i32,
    pub user_id: i32,
}

#[derive(Queryable)]
pub struct ChatInvites {
    pub id: i32,
    pub chat_id: i32,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub chat_id: i32,
    pub user_id: i32,
    pub message_text: String,
    pub sent_date: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct AddUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = chat_invites)]
pub struct Invite {
    pub chat_id: i32,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = group_chats)]
pub struct CreateChat {
    pub chat_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: i32,
}

#[derive(Insertable)]
#[diesel(table_name= messages)]
pub struct CreateMessage {
    pub chat_id: i32,
    pub user_id: i32,
    pub message_text: String,
    pub sent_at: chrono::NaiveDateTime,
}
