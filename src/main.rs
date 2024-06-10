use reqwest;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::time::SystemTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://google.com";

    let response = reqwest::get(url).await;

    let now = SystemTime::now();
    let timestamp = now.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();

    let status = match response {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("The website is up!");
                "up"
            } else {
                println!("The website is down! Status code: {}", resp.status());
                "down"
            }
        }
        Err(err) => {
            println!("Failed to check the website. Error: {}", err);
            "error"
        }
    };

    let conn = Connection::open("website_status.db")?;
    save_status_to_db(&conn, url, timestamp, status)?;

    Ok(())
}


fn save_status_to_db(conn: &Connection, url: &str, timestamp: u64, status: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO status (url, timestamp, status) VALUES (?1, ?2, ?3)",
        params![url, timestamp, status],
    )?;

    Ok(())
}
