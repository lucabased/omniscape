use anyhow::Error;
use rusqlite::{params, Connection, Transaction};
use crate::structs::user::User;
use crate::database::connection::_get_connection;



pub fn init_tiktok_layout() -> Result<(), Error> {
    // TODO: Make this use a input conn to make seperation easier in the future c:
    let sql_query = "
    CREATE TABLE tiktok_users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

        tiktok_id TEXT,
        unique_id TEXT,
        sec_uid TEXT,
        nickname TEXT,

        avatar_large TEXT,
        signature TEXT,
        
        is_private_account BOOLEAN,
        is_secret BOOLEAN,
        is_ftc BOOLEAN,
        is_tiktok_seller BOOLEAN,
        is_verified BOOLEAN
    )
    ";
    let mut conn = _get_connection();
    let transaction = conn.transaction();
    match transaction {
        Ok(trans) => {
            println!("Creating database layout...");
            trans.execute(sql_query, []).unwrap();
            let commit = trans.commit();
            match commit {
                Ok(()) => {
                    println!("Database layout has been committed.");
                },
                Err(e) => {
                    println!("vallah krise leichte beute => {}", e);
                }
            }
        },
        Err(e) => {
            println!("Ouchy, can't perform transaction (DB create) => {}", e);
        }
    }
    Ok(())
}
// unique_id, signature, avatar_large, is_ftc, is_tiktok_seller, is_verified

pub fn create_user(user: &User, transaction: Result<Transaction, rusqlite::Error>) -> Result<(), Error> {

    let sql_query = "INSERT INTO tiktok_users (
        tiktok_id,
        nickname,
        sec_uid,
        is_private_account,
        is_secret,
        unique_id,
        signature,
        avatar_large,
        is_ftc,
        is_tiktok_seller,
        is_verified
    )
        VALUES (
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?
        )";
    match &transaction {

        Ok(trans) => {
            #[warn(unused_variables)]
            let mut stmt = trans.prepare(sql_query)?;
             // unique_id, signature, avatar_large, is_ftc, is_tiktok_seller, is_verified

            stmt.execute(params![
                user.info.id,
                user.info.nickname,
                user.info.sec_uid,
                user.info.is_private,
                user.info.is_secret,
                user.info.unique_id,
                user.info.signature,
                user.info.avatar_large,
                user.info.is_ftc,
                user.info.is_tiktok_seller,
                user.info.is_verified
            ]).unwrap();
        },
        Err(e) => {
            println!("Error constructing transaction (user-create) => {}", e)
        }
    }
        transaction.unwrap().commit().unwrap();
    Ok(())
}

pub fn user_exists(secuid: &String) -> Result<bool, Error>{
    let conn: Connection = _get_connection();
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM tiktok_users WHERE tiktok_id = ?").unwrap();

    let count: i64 = stmt.query_row(params![secuid], |row| row.get(0)).unwrap();
    Ok(count > 0)
}