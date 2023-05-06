use actix_web::{get, post, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use super::DbPool;
use crate::models::{AddUser, User};

#[get("/users")]
async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    use super::schema::users::dsl::*;

    let connection = &mut pool.get().unwrap();
    let results = users.load::<User>(connection);

    match results {
        Ok(results) => HttpResponse::Ok().json(results.first().unwrap()),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[post("/users/add-user")]
async fn add_user(user: web::Json<AddUser>, pool: web::Data<DbPool>) -> impl Responder {
    use super::schema::users::dsl::*;

    //AddUser contains a plain text password well
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(user.password.as_bytes(), salt.as_salt())
        .unwrap();

    let connection = &mut pool.get().unwrap();

    let result = users
        .filter(username.eq(user.username.clone()))
        .or_filter(email.eq(user.email.clone()))
        .select(user_id)
        .first::<i32>(connection);
    if result.is_ok() {
        return HttpResponse::BadRequest().body("User already exists");
    }

    let result = diesel::insert_into(users)
        .values(AddUser {
            username: user.username.clone(),
            email: user.email.clone(),
            password: hash.to_string(),
        })
        .execute(connection);
    match result {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(error) => HttpResponse::BadRequest().body(format!("Failed to add user, {}", error)),
    }
}

#[actix_web::test]
async fn test_get_users() {
    let mut app = actix_web::test::init_service(actix_web::App::new().service(get_users)).await;
    let req = actix_web::test::TestRequest::with_uri("/users").to_request();

    let resp = actix_web::test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
}
