use reqwest;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::time::{Duration, SystemTime};
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let url = "https://google.com";

        let response = reqwest::get(url).await;

        let now = SystemTime::now();
        let timestamp = now.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();

        let status = match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("{} is up!", url);
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

        let conn = Connection::open("uptime_monitor.db")?;
        save_status_to_db(&conn, url, timestamp, status)?;

        time::sleep(Duration::from_secs(60)).await;
    }
}

fn save_status_to_db(conn: &Connection, url: &str, timestamp: u64, status: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO status (url, timestamp, status) VALUES (?1, ?2, ?3)",
        params![url, timestamp, status],
    )?;

    Ok(())
}
