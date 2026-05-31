use sqlite::State;
use std::error::Error;

pub fn database_open() -> Result<sqlite::Connection, Box<dyn Error>> {
    let db = sqlite::open("upc_scan.db")?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS upc_codes \
            (upc_code INTEGER NOT NULL, \
            upc_code_type INTEGER NOT NULL, \
            added_timestamp DATETIME NOT NULL);",
    )?;
    db.execute("DROP INDEX IF EXISTS upc_code_ndx;")?;
    db.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS upc_code_unique_ndx ON upc_codes \
            (upc_code, upc_code_type);",
    )?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS logs \
            (log_timestamp DATETIME NOT NULL, \
            log_text TEXT);",
    )?;
    db.execute(
        "CREATE INDEX IF NOT EXISTS log_time_ndx ON logs \
            (log_timestamp);",
    )?;
    Ok(db)
}

pub fn database_insert_logs(db: &sqlite::Connection, log_text: &str) -> Result<(), Box<dyn Error>> {
    let mut statement =
        db.prepare("INSERT INTO logs (log_timestamp, log_text) VALUES (CURRENT_TIMESTAMP, ?);")?;
    statement.bind((1, log_text))?;
    statement.next()?;
    Ok(())
}

pub fn database_upc_insert(
    db: &sqlite::Connection,
    upc_code: i64,
    upc_type: i32,
) -> Result<bool, Box<dyn Error>> {
    let mut statement = db.prepare(
        "INSERT OR IGNORE INTO upc_codes (upc_code, upc_code_type, added_timestamp) \
            VALUES (?, ?, CURRENT_TIMESTAMP);",
    )?;
    statement.bind((1, upc_code))?;
    statement.bind((2, upc_type as i64))?;
    statement.next()?;
    Ok(db.change_count() > 0)
}

pub fn database_upc_known_count(db: &sqlite::Connection) -> Result<i64, Box<dyn Error>> {
    let mut statement = db.prepare("SELECT count(*) AS total_found FROM upc_codes;")?;
    if statement.next()? == State::Row {
        return Ok(statement.read::<i64, _>("total_found")?);
    }
    Ok(0)
}
