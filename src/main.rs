mod tiktok;
mod structs;
mod http;
mod instagram;
mod tui;
mod database;

use anyhow::{Error, Result};
use rusqlite::Connection;
use tiktok::scraper::start_scraper;
use database::tiktok::init_tiktok_layout;
use database::util::check_table_exists;
use crate::database::connection::_get_connection;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // TODO: Load all already checked users into RAM/HashMap whenever program starts.
    let conn: Connection = _get_connection();

    // Check if tiktok_users table exists, create if not 
    // TODO: Make this safer :)
    if !check_table_exists(&conn, "tiktok_users").unwrap()  {
        println!("TikTok layout not found in Database => Creating layout.");
        init_tiktok_layout().unwrap();
    }
    start_scraper("MS4wLjABAAAAVMAY-_FS3V_ejpJor9K--GQ_FAcvaj4Gy-4O8ESU0OmwgSxP2N4eFbwy6X4KzMqL".to_string()).await;



    Ok(())
    // EOF

}
