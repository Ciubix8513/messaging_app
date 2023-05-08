use actix_web::{post, web, HttpResponse, Responder};
use common_structs::{GetInvites, SendInvite};
use diesel::{ExpressionMethods, JoinOnDsl, JoinTo, QueryDsl, RunQueryDsl};

use crate::{models::Invite, schema, utils::is_logged_in, DbPool};

#[post("/invites/send")]
pub async fn send_invite(
    invite: web::Json<SendInvite>,
    session: actix_session::Session,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let sender_id = is_logged_in(&session);
    if sender_id.is_err() {
        return HttpResponse::Unauthorized().body("");
    }
    let sender_id: i32 = sender_id.unwrap();
    let connection = &mut pool.get().unwrap();

    //Check if the user is in the chat they want invite someone to
    {
        use crate::schema::group_chat_members::dsl::*;

        let result: Result<usize, _> = group_chat_members
            .filter(user_id.eq(sender_id))
            .filter(chat_id.eq(invite.chat_id))
            .execute(connection);
        match result {
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            Ok(0) => return HttpResponse::Unauthorized().body(""),
            _ => (),
        }
    }
    //Check if invitee exists
    //Invitee IS a real word btw
    {
        use crate::schema::users::dsl::*;

        let result: Result<usize, _> = users.find(invite.recipient_id).execute(connection);
        match result {
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            Ok(0) => return HttpResponse::BadRequest().body("Invited user does not exist"),
            _ => (),
        }
    }

    //Create invite
    let s_id = sender_id;
    {
        use crate::schema::chat_invites::dsl::*;

        let result = diesel::insert_into(chat_invites)
            .values(Invite {
                chat_id: invite.chat_id,
                sender_id: s_id,
                recipient_id: invite.recipient_id,
                created_at: chrono::Local::now().naive_utc(),
            })
            .execute(connection);

        match result {
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            _ => HttpResponse::Ok().body(""),
        }
    }
}

#[actix_web::get("/invites/get")]
async fn get_invites(session: actix_session::Session, pool: web::Data<DbPool>) -> impl Responder {
    let sender_id = is_logged_in(&session);
    if sender_id.is_err() {
        return HttpResponse::Unauthorized().body("");
    }
    let u_id: i32 = sender_id.unwrap();
    let connection = &mut pool.get().unwrap();

    {
        use crate::schema::chat_invites::dsl as inv;
        use crate::schema::group_chats::dsl as gchat;
        use crate::schema::users::dsl as us;

        let result: Result<Vec<(i32, i32, String, String, chrono::NaiveDateTime)>, _> =
            inv::chat_invites
                .filter(inv::recipient_id.eq(u_id))
                .inner_join(us::users.on(inv::sender_id.eq(us::user_id)))
                .inner_join(gchat::group_chats)
                .select((
                    inv::invite_id,
                    inv::chat_id,
                    gchat::chat_name,
                    us::username,
                    inv::created_at,
                ))
                .load(connection);
        match result {
            Ok(value) => HttpResponse::Ok().json(
                value
                    .iter()
                    .map(|value| GetInvites {
                        invite_id: value.0,
                        chat_id: value.1,
                        chat_name: value.2.clone(),
                        sender_name: value.3.clone(),
                        created_at: value.4,
                    })
                    .collect::<Vec<GetInvites>>(),
            ),
            Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
        }
    }
}
