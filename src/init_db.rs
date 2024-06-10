use rusqlite::{Connection, Result};

pub fn initialize_database() -> Result<()> {
    // Open a connection to the SQLite database
    let conn = Connection::open("website_status.db")?;

    // Create the table if it does not exist
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
