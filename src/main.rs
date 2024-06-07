use reqwest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://google.com";

    let response = reqwest::get(url).await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("The website is up!");
                println!("Status code: {}", resp.status());
            } else {
                println!("The website is down! Status code: {}", resp.status());
            }
        }
        Err(err) => {
            println!("Failed to check the website. Error: {}", err);
        }
    }

    Ok(())
}
