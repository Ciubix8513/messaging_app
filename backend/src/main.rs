#![allow(unused_imports)]
use actix_web::{
    get,
    http::StatusCode,
    post, test,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use diesel::RunQueryDsl;
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::{env, fs::File, io::BufReader};

use crate::models::User;

pub mod models;
pub mod schema;
pub mod utils;

type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>;

#[get("/users")]
async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    use self::schema::users::dsl::*;

    let connection = &mut pool.get().unwrap();
    let results = users.load::<User>(connection);

    match results {
        Ok(results) => HttpResponse::Ok().json(results.first().unwrap()),
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
    let pool = utils::establish_connection();

    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder.set_private_key_file(
    //     "/home/luna/Projects/corporate_network/target/debug/private.key",
    //     SslFiletype::PEM,
    // )?;
    // builder.set_certificate_chain_file(
    //     "/home/luna/Projects/corporate_network/target/debug/certificate.crt",
    // )?;
    println!("Running server on {}:{}", ip, port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get_users)
    })
    .bind((ip, port))?
    // .bind_openssl((ip, port), builder)?
    .run()
    .await
}
