use actix_web::{post, web, HttpResponse, Responder};
use chrono::Local;
use common_structs::SendMessage;
use diesel::{QueryDsl, RunQueryDsl};

use crate::{models::CreateMessage, utils::is_logged_in, DbPool};

#[post("/messages/send")]
pub async fn send_message(
    session: actix_session::Session,
    pool: web::Data<DbPool>,
    message: web::Json<SendMessage>,
) -> impl Responder {
    let sender_id = is_logged_in(&session);
    if sender_id.is_err() {
        return HttpResponse::Unauthorized().body("");
    }
    let u_id: i32 = sender_id.unwrap();

    let connection = &mut pool.get().unwrap();
    //Check if the user in the chat
    {
        use crate::schema::group_chat_members::dsl::*;

        let result = group_chat_members
            .find((message.chat_id, u_id))
            .execute(connection);
        match result {
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            Ok(0) => return HttpResponse::Unauthorized().body(""),
            _ => (),
        }
    }
    {
        use crate::schema::messages::dsl::*;
        let result = diesel::insert_into(messages)
            .values(CreateMessage {
                chat_id: message.chat_id,
                user_id: u_id,
                message_text: message.text.clone(),
                sent_at: Local::now().naive_utc(),
            })
            .execute(connection);
        match result {
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            Ok(_) => HttpResponse::Ok().body(""),
        }
    }
}
