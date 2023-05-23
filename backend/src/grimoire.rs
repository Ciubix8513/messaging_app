use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static FILE_LOCATION: Lazy<PathBuf> =
    Lazy::new(|| std::env::current_dir().unwrap().join("./files"));
pub const USER_ID_KEY: &str = "user_id";
pub const USERNAME_KEY: &str = "username";
pub const PUBLIC_KEY_KEY: &str = "private_key";
pub const COOKIE_KEY_FILENAME: &str = "Cookie.key";
pub const OLD_KEY_FILENAME: &str = "Old.key";
