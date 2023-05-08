use actix_web::{post, web, HttpResponse, Responder};
use diesel::RunQueryDsl;

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
