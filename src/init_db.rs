use rusqlite::{Connection, Result};

pub fn initialize_database() -> Result<()> {
    let conn = Connection::open("uptime_monitor.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS status (
            id INTEGER PRIMARY KEY,
            url TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

fn main() {
    match initialize_database() {
        Ok(_) => println!("Database initialized."),
        Err(err) => eprintln!("Failed to initialize the database. Error: {}", err),
    }
}
