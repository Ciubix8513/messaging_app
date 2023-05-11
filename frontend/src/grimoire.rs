#![allow(unused)]
use once_cell::sync::Lazy;
use reqwest::Url;

pub static API_URL: Lazy<Url> = Lazy::new(|| Url::parse("https://api.ciubix.xyz").unwrap());

pub static AUTH_LOGIN: Lazy<Url> = Lazy::new(|| API_URL.join("/auth/login").unwrap());
pub static AUTH_LOGOUT: Lazy<Url> = Lazy::new(|| API_URL.join("/auth/logout").unwrap());
pub static AUTH_CHANGE_PASSWORD: Lazy<Url> =
    Lazy::new(|| API_URL.join("/auth/change-password").unwrap());
pub static CHATS_CREATE: Lazy<Url> = Lazy::new(|| API_URL.join("/chats/create").unwrap());
pub static CHATS_EXIT: Lazy<Url> = Lazy::new(|| API_URL.join("/chats/exit").unwrap());
pub static CHATS_GET: Lazy<Url> = Lazy::new(|| API_URL.join("/chats/get").unwrap());
pub static INVITES_SEND: Lazy<Url> = Lazy::new(|| API_URL.join("/invites/send").unwrap());
pub static INVITES_GET: Lazy<Url> = Lazy::new(|| API_URL.join("/invites/get").unwrap());
pub static INVITES_REJECT: Lazy<Url> = Lazy::new(|| API_URL.join("/invites/reject").unwrap());
pub static INVITES_ACCEPT: Lazy<Url> = Lazy::new(|| API_URL.join("/invites/accept").unwrap());
pub static MESSAGES_SEND: Lazy<Url> = Lazy::new(|| API_URL.join("/messages/send").unwrap());
pub static MESSAGES_GET: Lazy<Url> = Lazy::new(|| API_URL.join("/messages/get").unwrap());
pub static USERS_ADD_USER: Lazy<Url> = Lazy::new(|| API_URL.join("/users/add-user").unwrap());

pub static REFRESH_TIME: u64 = 2;
