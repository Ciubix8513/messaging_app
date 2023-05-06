#![allow(unused_imports)]
use actix_web::{get, http::StatusCode, test, App, HttpResponse, HttpServer, Responder};
use diesel::RunQueryDsl;
use dotenvy::dotenv;
use std::env;

use crate::models::User;

pub mod models;
pub mod schema;
pub mod utils;

#[get("/users")]
async fn get_users() -> impl Responder {
    use self::schema::users::dsl::*;

    let connection = utils::establish_connection();
    if connection.is_err() {
        return HttpResponse::InternalServerError().body(connection.err().unwrap().to_string());
    }
    let connection = &mut connection.unwrap();

    let results = users.load::<User>(connection);

    match results {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[actix_web::test]
async fn test_get_users() {
    let mut app = test::init_service(App::new().service(get_users)).await;
    let req = test::TestRequest::with_uri("/users").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let ip = env::var("IP_ADDRESS").expect("Ip adress should be set");
    let port = env::var("PORT")
        .expect("Port must be set")
        .parse()
        .expect("Invalid port number");
    HttpServer::new(|| App::new().service(get_users))
        .bind((ip, port))?
        .run()
        .await
}
