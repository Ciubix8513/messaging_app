use diesel::{mysql::MysqlConnection, Connection, ConnectionError};
use dotenvy::dotenv;
use std::env;
pub fn establish_connection() -> Result<MysqlConnection, ConnectionError> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB url must be set");
    MysqlConnection::establish(&db_url) //.unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
