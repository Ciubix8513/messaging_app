use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use super::DbPool;
use crate::{models::AddUser, utils};

#[get("/users/{id}/name")]
pub async fn get_user_with_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    use super::schema::users::dsl::*;
    let id = path.into_inner();

    let connection = &mut pool.get().unwrap();
    let results: Result<String, _> = users
        .filter(user_id.eq(id))
        .select(username)
        .first(connection);

    match results {
        Ok(res) => HttpResponse::Ok().body(res),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::BadRequest().body(format!("No user with id {}", id))
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/users/add-user")]
pub async fn add_user(user: web::Json<AddUser>, pool: web::Data<DbPool>) -> impl Responder {
    use super::schema::users::dsl::*;

    //AddUser contains a plain text password well
    let hash = utils::hash_password(&user.password);
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
            password: hash,
        })
        .execute(connection);
    match result {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(error) => HttpResponse::BadRequest().body(format!("Failed to add user, {}", error)),
    }
}

// #[actix_web::test]
// async fn test_get_users() {
//     let mut app = actix_web::test::init_service(actix_web::App::new().service(get_users)).await;
//     let req = actix_web::test::TestRequest::with_uri("/users").to_request();

//     let resp = actix_web::test::call_service(&mut app, req).await;
//     assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
// }
