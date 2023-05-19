use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{Local, NaiveDateTime};
use common_lib::{GetMessage, SendMessage};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_derive::Deserialize;

use crate::{models::CreateMessage, utils::is_logged_in, DbPool};

#[derive(Deserialize)]
pub struct Param {
    pub id: i32,
}

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

#[get("/messages/get")]
pub async fn get_messages(
    session: actix_session::Session,
    pool: web::Data<DbPool>,
    web::Query(param): web::Query<Param>,
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
            .find((param.id, u_id))
            .execute(connection);
        match result {
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            Ok(0) => return HttpResponse::Unauthorized().body(""),
            _ => (),
        }
    }
    {
        use crate::schema::messages::dsl as msg;
        use crate::schema::users::dsl as us;

        let result: Result<Vec<(i32, i32, String, String, NaiveDateTime)>, _> = msg::messages
            .filter(msg::chat_id.eq(param.id))
            .order_by(msg::sent_at.desc())
            .inner_join(us::users)
            .select((
                msg::message_id,
                msg::user_id,
                us::username,
                msg::message_text,
                msg::sent_at,
            ))
            .load(connection);
        match result {
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            Ok(values) => HttpResponse::Ok().json(
                values
                    .iter()
                    .map(|val| GetMessage {
                        message_id: val.0,
                        user_id: val.1,
                        username: val.2.clone(),
                        message_text: val.3.clone(),
                        sent_at: val.4,
                    })
                    .collect::<Vec<_>>(),
            ),
        }
    }
}
