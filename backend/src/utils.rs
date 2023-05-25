use argon2::{Argon2, PasswordHasher};
use diesel::{
    mysql::MysqlConnection,
    r2d2::{ConnectionManager, Pool},
};
use password_hash::{rand_core::OsRng, SaltString};
use rand::Rng;
use std::env;

use crate::grimoire;
pub fn establish_connection() -> Pool<ConnectionManager<MysqlConnection>> {
    let db_url = env::var("DATABASE_URL").expect("DB url must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build pool")
}

pub fn is_logged_in(session: &actix_session::Session) -> Result<i32, String> {
    match session.get(grimoire::USER_ID_KEY) {
        Ok(Some(id)) => Ok(id),
        Ok(None) => Err("No value".to_string()),
        Err(e) => Err(format!("{e}")),
    }
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), salt.as_salt())
        .unwrap()
        .to_string()
}

pub fn generate_uuid() -> u128 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
