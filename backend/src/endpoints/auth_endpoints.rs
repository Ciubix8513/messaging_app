use actix_web::{delete, post, web, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use common_lib::{ChangePassword, Login, UserData};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use password_hash::Encoding;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};

use crate::utils::{hash_password, is_logged_in};
use crate::{grimoire, DbPool};

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
            let mut rng = rand::thread_rng();
            let bits = 2048;

            let priv_key = rsa::RsaPrivateKey::new(&mut rng, bits).unwrap();
            let pub_key = rsa::RsaPublicKey::from(&priv_key);

            session.renew();
            session
                .insert(grimoire::USER_ID_KEY, user.1)
                .expect("Could not insert user id into session");
            session
                .insert(grimoire::USERNAME_KEY, login_info.username.clone())
                .expect("Could not insert user id into session");
            session
                .insert(
                    grimoire::PUBLIC_KEY_KEY,
                    pub_key
                        .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
                        .unwrap()
                        .to_string(),
                )
                .expect("Could not insert private key");

            //User also gets a cookie
            HttpResponse::Ok().json(UserData {
                username: login_info.username.clone(),
                user_id: user.1,
                private_key: priv_key
                    .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
                    .unwrap()
                    .to_string(),
            })
        }
        Err(_) => HttpResponse::BadRequest().body("Invalid username or password"),
    }
}

#[delete("/auth/logout")]
pub async fn logout(session: actix_session::Session) -> impl Responder {
    match is_logged_in(&session) {
        Ok(_) => {
            session.purge();
            HttpResponse::Ok()
        }
        Err(_) => HttpResponse::Unauthorized(),
    }
}

#[post("/auth/change-password")]
pub async fn change_passowrd(
    session: actix_session::Session,
    new_password: web::Json<ChangePassword>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let id = is_logged_in(&session);
    if id.is_err() {
        return HttpResponse::Unauthorized().body("");
    }
    let id = id.unwrap();
    let connection = &mut pool.get().unwrap();
    let password = {
        use crate::schema::users::dsl::*;
        let result: Result<String, _> = users
            .filter(user_id.eq(id))
            .select(password)
            .first(connection);
        if result.is_err() {
            return HttpResponse::InternalServerError().body(result.err().unwrap().to_string());
        }
        result.unwrap()
    };
    let password = PasswordHash::parse(&password, Encoding::B64).unwrap();
    if Argon2::default()
        .verify_password(new_password.old_password.as_bytes(), &password)
        .is_err()
    {
        return HttpResponse::BadRequest().body("Wrong password");
    }

    let new_password = hash_password(&new_password.new_password);
    {
        use crate::schema::users::dsl::*;
        let result: Result<usize, _> = diesel::update(users.find(id))
            .set(password.eq(new_password))
            .execute(connection);
        match result {
            Ok(1) => HttpResponse::Ok().body(""),
            Ok(_) => HttpResponse::InternalServerError().body(""),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }
}
