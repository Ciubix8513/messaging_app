use std::{
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use iced::{Application, Settings};
use once_cell::sync::Lazy;
use reqwest::blocking::Client;

mod grimoire;
mod login;
mod main_window;

fn cookie_path() -> PathBuf {
    let base_dirs = directories::BaseDirs::new().unwrap();
    base_dirs
        .cache_dir()
        .join(".messanger_app_frontend_cookies.json")
}

fn get_cookie_store_mutex() -> reqwest_cookie_store::CookieStoreMutex {
    let cookie_store = {
        match std::fs::File::open(cookie_path()) {
            Ok(file) => {
                let file = std::io::BufReader::new(file);
                reqwest_cookie_store::CookieStore::load_json(file)
                    .unwrap_or_else(|_| reqwest_cookie_store::CookieStore::default())
            }
            Err(_) => reqwest_cookie_store::CookieStore::default(),
        }
    };
    reqwest_cookie_store::CookieStoreMutex::new(cookie_store)
}
pub static CLIENT: Lazy<Arc<Mutex<Option<Client>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
pub static COOKIE_STORE: Lazy<Arc<reqwest_cookie_store::CookieStoreMutex>> =
    Lazy::new(|| Arc::new(get_cookie_store_mutex()));

fn main() -> Result<(), iced::Error> {
    *CLIENT.lock().unwrap() = Some(
        Client::builder()
            .user_agent(concat!(
                "messenger_app frontend / ",
                env!("CARGO_PKG_VERSION")
            ))
            // .cookie_store(true)
            .cookie_provider(Arc::clone(&COOKIE_STORE))
            .build()
            .unwrap(),
    );

    let mut settings = Settings::default();
    settings.window.resizable = false;
    settings.window.size = (320, 600);
    main_window::MainForm::run(settings)?;
    println!("Quit");

    let mut writer = std::fs::File::create(cookie_path())
        .map(std::io::BufWriter::new)
        .unwrap();
    let store = COOKIE_STORE.lock().unwrap();
    store.save_json(&mut writer).unwrap();
    println!("{}", cookie_path().display());
    Ok(())
}
