#![allow(
    clippy::type_complexity,
    clippy::wildcard_imports,
    clippy::unused_async,
    clippy::future_not_send,
    clippy::option_if_let_else
)]
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web::Data, App, HttpResponse, HttpServer, Responder};
use common_lib::encryption::{generate_aes_key, into_key};
use dotenvy::dotenv;
use std::{env, fs::File, io::Write};

use crate::endpoints::*;

mod encryption;
mod endpoints;
pub mod grimoire;
pub mod models;
pub mod schema;
mod utils;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>;

#[actix_web::get("/")]
async fn html_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let ip = env::var("IP_ADDRESS").expect("Ip adress should be set");
    let port = env::var("PORT")
        .expect("Port must be set")
        .parse()
        .expect("Invalid port number");

    let cookie_key = std::fs::read(grimoire::COOKIE_KEY_FILENAME);

    let secret_key = if let Ok(key) = cookie_key {
        Key::from(&key)
    } else {
        let k = Key::generate();
        let mut f = File::create(grimoire::COOKIE_KEY_FILENAME).unwrap();
        f.write_all(k.master()).unwrap();
        k
    };

    let pool = utils::establish_connection();

    let old_key = std::fs::read(grimoire::OLD_KEY_FILENAME)
        .ok()
        .map(|k| into_key(&k));
    let new_key = generate_aes_key();

    encryption::deploy(new_key, old_key, &pool);

    //Write key to use upon next start up
    std::fs::write(grimoire::OLD_KEY_FILENAME, new_key).unwrap();
    println!("Generated new key, please don't just leave it here");

    println!("Running server on {ip}:{port}");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            //Deployment key
            .app_data(Data::new(new_key))
            //Db pool
            .app_data(Data::new(pool.clone()))
            .service(html_page)
            .service(login)
            .service(logout)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .service(add_user)
            .service(get_user_with_id)
            .service(change_passowrd)
            .service(send_invite)
            .service(create_chat)
            .service(exit_chat)
            .service(get_invites)
            .service(reject_invite)
            .service(accept_invite)
            .service(send_message)
            .service(get_messages)
            .service(get_chats)
            .service(get_key)
            .service(upload_file)
            //Wrap "Wraps" all the registered services in itself
            .wrap(middleware::Logger::default())
    })
    //Fuck ssl i'm just gonna use cloudflare tunnels
    .bind((ip, port))?
    .run()
    .await
}
