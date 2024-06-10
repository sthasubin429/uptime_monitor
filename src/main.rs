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
        let url: &str = "https://asdasdasdasdasdasdasw2123.com";
        time::sleep(Duration::from_secs(10)).await;

        let response: reqwest::Response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(err) => {
                println!("Error: {}", err);

                let now: SystemTime = SystemTime::now();
                let timestamp: u64 = now.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
                let status: WebsiteStatus = WebsiteStatus {
                    url: url.to_string(),
                    timestamp,
                    status: false,
                    status_code: 0, // Set status code to 0 for DNS error
                };
                save_status_to_db(&status)?;

                continue;
            }
        };

        let now: SystemTime = SystemTime::now();
        let timestamp: u64 = now.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
        let is_up: bool = response.status().is_success();
        let status_code: u16 = response.status().as_u16();

        log_status_message(&url, is_up, status_code);

        let status: WebsiteStatus = WebsiteStatus {
            url: url.to_string(),
            timestamp,
            status: is_up,
            status_code,
        };
        save_status_to_db(&status)?;
    }
}

fn save_status_to_db(status: &WebsiteStatus) -> Result<()> {
    let conn: Connection = Connection::open("uptime_monitor.db")?;

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
