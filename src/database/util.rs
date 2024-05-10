use rusqlite::Connection;

pub fn check_table_exists(conn: &Connection, table_name: &str) -> Result<bool, anyhow::Error> {
    #[warn(unused_variables)]
    let mut stmt = conn.prepare("
        SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?
    ").unwrap();
    #[warn(unused_variables)]
    let count: i64 = stmt.query_row(&[&table_name], |row| row.get(0)).unwrap();

    Ok(count > 0)
}