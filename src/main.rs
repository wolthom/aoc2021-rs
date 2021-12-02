use dotenv;
use reqwest;
use std::env;
use std::error::Error;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    // Get token from environment
    dotenv::dotenv().ok();
    let token = env::var("AOC_SESSION_TOKEN")
        .expect("Session token (AOC_SESSION_TOKEN) not found in environment variables");

    // Set up cookie store
    let base_url: reqwest::Url = "https://adventofcode.com"
        .parse()
        .expect("Parsing AoC URL failed");
    let cookie_str = format!("session={}", token);
    let jar = reqwest::cookie::Jar::default();
    jar.add_cookie_str(&cookie_str, &base_url);

    // Create client with cookie
    let client = reqwest::blocking::Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()?;

    // Use client to get input data
    let resp = client
        .get("https://adventofcode.com/2021/day/1/input")
        .send()?;
    dbg!(resp.text());

    Ok(())
}
