#![allow(unused_imports)]
use actix_web::{web::Data, App, HttpServer};
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::{env, fs::File, io::BufReader};

use crate::user_endpoints::{add_user, get_users};

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
    println!("Running server on {}:{}", ip, port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get_users)
            .service(add_user)
    })
    .bind((ip, port))?
    // .bind_openssl((ip, port), builder)?
    .run()
    .await
}
