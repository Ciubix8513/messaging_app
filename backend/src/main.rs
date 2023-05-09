use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;

use crate::endpoints::*;

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
    let pool = utils::establish_connection();
    let secret_key = Key::generate();

    println!("Running server on {}:{}", ip, port);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
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
            //Wrap "Wraps" all the registered services in itself
            .wrap(middleware::Logger::default())
    })
    //Fuck ssl i'm just gonna use cloudflare tunnels
    .bind((ip, port))?
    .run()
    .await
}
