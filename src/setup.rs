use reqwest;
use std::env;
use std::error::Error;
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use std::fs::{create_dir, read_to_string, write};
use log::{info, error};


lazy_static! {
    static ref CLIENT: reqwest::blocking::Client = initialize_client();
}

pub fn get_data<T: AsRef<Path>>(day: usize, data_dir: T) -> Result<String, Box<dyn Error>>{
    // Validate day input
    if (day < 1) || (day > 25) {
        error!("Data for invalid day requested: {} Terminating... \n", day);
        return Err("Invalid day received. Please insert values between 1 and 25 (inclusive)".into());
    }
    info!{"Retrieving input data for day {}", day};

    // Validate or create data directory
    let data_dir = data_dir.as_ref();
    if !(data_dir.exists() && data_dir.is_dir()) {
        info!{"\tData directory ({:?}) does not exist. Creating...", data_dir};
        create_dir(data_dir)?;
    }

    // Create Path to input file of day
    let mut input = PathBuf::new();
    input.push(data_dir);
    input.push(format!("day{}.txt", day));
    
    // Check if input file exists
    if input.as_path().exists() &   & input.as_path().is_file() {
        info!{"\tData for day {} already downloaded. Reading from disk...", day};
        return Ok(read_to_string(input)?);
    }
    // If path exists but is not a file, it is a directory and the file can't be created
    if input.as_path().exists() {
        error!{"\tFile: {:?} already exists as a directory. Terminating... \n", input.as_path()};
        let msg = format!("{} already exists as a directory! Can't create file!", input.to_str().unwrap());
        return Err(msg.into());
    }
    // Download data, write it to file and return the data
    info!{"\tDownloading input data..."};
    let data = download_input(day)?;
    info!{"\tWriting input data to disk..."};
    write(input, &data)?;

    Ok(data)
}



fn download_input(day: usize) -> Result<String, Box<dyn Error>>{
    let url_string = format!("https://adventofcode.com/2021/day/{}/input", day);
    
    // Use client to get input data
    let resp = CLIENT
        .get(url_string)
        .send()?;
    
    let data = resp.text()?;

    Ok(data)
}

fn initialize_client() -> reqwest::blocking::Client {
    // Get token from environment
    let token = env::var("AOC_SESSION_TOKEN")
        .expect("Session token (AOC_SESSION_TOKEN) not found in environment variables");

    // Prepare session cookie header
    let mut hm = reqwest::header::HeaderMap::new();
    let cookie_str = format!("session={}", token);
    hm.insert(
        reqwest::header::COOKIE, 
        cookie_str.try_into().expect("Cookie string could not be parsed into header value")
    );

    // Create client with cookie
    let client = reqwest::blocking::Client::builder()
        .default_headers(hm)
        .build()
        .expect("Initializing reqwest client failed unexpectedly");
    client
}
