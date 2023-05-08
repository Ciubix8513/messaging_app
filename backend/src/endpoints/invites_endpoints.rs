use actix_web::{post, web, HttpResponse, Responder};
use common_structs::SendInvite;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{models::Invite, utils::is_logged_in, DbPool};

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

        let result: Result<usize, _> = users.find(invite.recepient_id).execute(connection);
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
                recipient_id: invite.recepient_id,
                created_at: chrono::Local::now().naive_utc(),
            })
            .execute(connection);

        match result {
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            _ => HttpResponse::Ok().body(""),
        }
    }
}
