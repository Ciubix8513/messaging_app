#![allow(unused_imports)]
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web::Data, App, HttpServer};
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::{env, fs::File, io::BufReader};

use crate::auth_endpoints::{login, logout};
use crate::user_endpoints::{add_user, get_users};

mod auth_endpoints;
pub mod keys;
pub mod models;
pub mod schema;
mod user_endpoints;
mod utils;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>;

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
    let secret_key = Key::generate();

    println!("Running server on {}:{}", ip, port);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(login)
            .service(logout)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .service(add_user)
            .service(get_users)
            //Wrap "Wraps" all the registered services in itself
            .wrap(middleware::Logger::default())
    })
    .bind((ip, port))?
    // .bind_openssl((ip, port), builder)?
    .run()
    .await
}
