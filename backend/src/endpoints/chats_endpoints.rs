use actix_web::{get, post, web, HttpResponse, Responder};
use common_structs::GetChat;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};

use crate::{
    models::{CreateChat, GroupChatMember},
    utils::is_logged_in,
    DbPool,
};

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

    let values = CreateChat {
        chat_name: name.into_inner(),
        created_at: chrono::Local::now().naive_utc(),
        created_by: sender_id,
    };

    let result = {
        use crate::schema::group_chats::dsl::*;

        diesel::insert_into(group_chats)
            .values(values.clone())
            .execute(connection)
    };
    match result {
        Err(_) => return HttpResponse::InternalServerError(),
        Ok(_) => (),
    }
    let result = {
        use crate::schema::group_chat_members::dsl as gcm;
        use crate::schema::group_chats::dsl as gc;

        let id: Result<i32, _> = gc::group_chats
            .select(gc::chat_id)
            .filter(gc::created_at.eq(values.created_at))
            .filter(gc::created_by.eq(values.created_by))
            .first(connection);
        if id.is_err() {
            return HttpResponse::InternalServerError();
        }

        diesel::insert_into(gcm::group_chat_members)
            .values(GroupChatMember {
                chat_id: id.unwrap(),
                user_id: sender_id,
            })
            .execute(connection)
    };
    if result.is_err() {
        return HttpResponse::InternalServerError();
    }
    HttpResponse::Ok()
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

//HOW DID I FORGET TO ADD THIS?
#[get("/chats/get")]
async fn get_chats(pool: web::Data<DbPool>, session: actix_session::Session) -> impl Responder {
    let sender_id = is_logged_in(&session);
    if sender_id.is_err() {
        return HttpResponse::Unauthorized().body("");
    }
    let sender_id: i32 = sender_id.unwrap();

    let connection = &mut pool.get().unwrap();

    {
        use crate::schema::group_chat_members::dsl as gcm;
        use crate::schema::group_chats::dsl as gc;
        use crate::schema::users::dsl as u;

        let result: Result<Vec<(i32, String, i32, String)>, _> = gcm::group_chat_members
            .filter(gcm::chat_id.eq(sender_id))
            .inner_join(gc::group_chats)
            .inner_join(u::users.on(u::user_id.eq(gc::created_by)))
            .select((gcm::chat_id, gc::chat_name, u::user_id, u::username))
            .load(connection);
        match result {
            Ok(values) => HttpResponse::Ok().json(
                values
                    .iter()
                    .map(|v| GetChat {
                        chat_id: v.0,
                        chat_name: v.1.clone(),
                        creator_id: v.2,
                        creator_name: v.3.clone(),
                    })
                    .collect::<Vec<_>>(),
            ),
            Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
        }
    }
}
