use reqwest;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::time::{Duration, SystemTime};
use tokio::time;

struct WebsiteStatus {
    url: String,
    timestamp: u64,
    status: bool,
    status_code: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let url = "https://subinshrestha.com.np";

        let response = reqwest::get(url).await?;

        let now = SystemTime::now();
        let timestamp = now.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();

        let is_up = response.status().is_success();

        let status_code = response.status().as_u16();

        log_status_message(&url, is_up, status_code);

        let status = WebsiteStatus {
            url: url.to_string(),
            timestamp,
            status: is_up,
            status_code,
        };
        save_status_to_db(&status)?;

        time::sleep(Duration::from_secs(1)).await;
    }
}

fn save_status_to_db(status: &WebsiteStatus) -> Result<()> {
    let conn = Connection::open("uptime_monitor.db")?;

    conn.execute(
        "INSERT INTO status (url, timestamp, status, status_code) VALUES (?1, ?2, ?3, ?4)",
        params![
            status.url,
            status.timestamp,
            status.status,
            status.status_code
        ],
    )?;

    Ok(())
}

fn log_status_message(url: &str, is_up: bool, status_code: u16) {
    match (is_up, status_code) {
        (true, _) => println!("{} is Up with status {};", url, status_code),
        (false, _) => println!("{} is Down with status {};", url, status_code),
    }
}
