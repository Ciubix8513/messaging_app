use actix_web::{delete, post, web, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use common_structs::{Login, UserData};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use password_hash::Encoding;

use crate::{keys, DbPool};

#[post("/auth/login")]
pub async fn login(
    login_info: web::Json<Login>,
    pool: web::Data<DbPool>,
    session: actix_session::Session,
) -> impl Responder {
    let connection = &mut pool.get().unwrap();

    let result = {
        use crate::schema::users::dsl::*;
        users
            .filter(username.eq(login_info.username.clone()))
            .select((password, user_id))
            .first::<(String, i32)>(connection)
    };
    if result.is_err() {
        return HttpResponse::BadRequest().body("Invalid username or password");
    }
    let user = result.unwrap();
    let password = PasswordHash::parse(&user.0, Encoding::B64).unwrap();
    let result = Argon2::default().verify_password(login_info.password.as_bytes(), &password);

    match result {
        Ok(_) => {
            session.renew();
            session
                .insert(keys::USER_ID_KEY, user.1)
                .expect("Could not insert user id into session");
            session
                .insert(keys::USERNAME_KEY, login_info.username.clone())
                .expect("Could not insert user id into session");
            //User also gets a cookie
            HttpResponse::Ok().json(UserData {
                username: login_info.username.clone(),
                user_id: user.1,
            })
        }
        Err(_) => HttpResponse::BadRequest().body("Invalid username or password"),
    }
}

