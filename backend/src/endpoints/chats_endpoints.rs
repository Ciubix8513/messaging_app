use actix_web::{post, web, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{models::CreateChat, utils::is_logged_in, DbPool};

#[post("/chats/create")]
async fn create_chat(
    name: web::Json<String>,
    pool: web::Data<DbPool>,
    session: actix_session::Session,
) -> impl Responder {
    let sender_id = is_logged_in(&session);
    if sender_id.is_err() {
        return HttpResponse::Unauthorized();
    }
    let sender_id: i32 = sender_id.unwrap();

    let connection = &mut pool.get().unwrap();
    let result = {
        use crate::schema::group_chats::dsl::*;

        diesel::insert_into(group_chats)
            .values(CreateChat {
                chat_name: name.into_inner(),
                created_at: chrono::Local::now().naive_utc(),
                created_by: sender_id,
            })
            .execute(connection)
    };

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[post("/chats/exit")]
async fn exit_chat(
    id: web::Json<i32>,
    pool: web::Data<DbPool>,
    session: actix_session::Session,
) -> impl Responder {
    let sender_id = is_logged_in(&session);
    if sender_id.is_err() {
        return HttpResponse::Unauthorized().body("");
    }
    let sender_id: i32 = sender_id.unwrap();
    let id = id.into_inner();

    let connection = &mut pool.get().unwrap();

    //Check if the user is in the chat they want to leave
    {
        use crate::schema::group_chat_members::dsl::*;

        let result: Result<usize, _> = group_chat_members.find((id, sender_id)).execute(connection);
        match result {
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            Ok(0) => return HttpResponse::BadRequest().body(format!("No chat with id {}", id)),
            _ => (),
        }
    }
    {
        use crate::schema::group_chat_members::dsl::*;
        let result = diesel::delete(group_chat_members)
            .filter(user_id.eq(sender_id))
            .filter(chat_id.eq(id))
            .execute(connection);
        match result {
            Ok(_) => HttpResponse::Ok().body(""),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}
