use dotenv::dotenv;
use std::env;
use rusqlite::Connection;

pub fn _get_connection() -> rusqlite::Connection {
    dotenv().ok();
    let database_file = env::var("DB_FILE").expect("bruh");
    Connection::open(database_file).unwrap()
}