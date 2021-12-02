use dotenv;
use reqwest;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let token = env::var("AOC_SESSION_TOKEN")
        .expect("Session token (AOC_SESSION_TOKEN) not found in environment variables");
    let client = reqwest::blocking::Client::builder().build()?;
    Ok(())
}
