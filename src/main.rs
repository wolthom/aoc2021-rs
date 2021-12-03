use std::error::Error;
use dotenv;
use structopt::StructOpt;
use aoc2021_rs::{setup::get_data, cli::Opt, solutions::SOLS};

const DATA_DIR: &str = "./data";

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    env_logger::init();
    
    let opt = Opt::from_args();
    if opt.all {
        SOLS.keys().copied().map(|k,| run(k)).collect::<Result<Vec<_>, _>>()?;
    } else if opt.day.is_some() && opt.part.is_some() {
        let key = (opt.day.unwrap(), opt.part.unwrap());
        run(key)?;
    } else {
        let msg = format!("Either specify '-a' or '-d' AND '-p'");
        return Err(msg.into());
    }

    Ok(())
}

fn run(key: (usize, usize)) -> Result<(), Box<dyn Error>>{
    let f = SOLS.get(&key);
    if let Some(f) = f {
        let data = get_data(key.0, DATA_DIR)?;
        f(data);
    } else {
        let msg = format!("Solution for day {0} part {1} does not exist yet!", key.0, key.1);
        return Err(msg.into());
    }
    Ok(())
}
