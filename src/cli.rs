use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2021-rs", about = "Advent of Code 2021 solutions from Wolf Thomsen")]
pub struct Opt {
    #[structopt(short, long)]
    pub all: bool,

    #[structopt(short, long)]
    pub day: Option<usize>,

    #[structopt(short, long)]
    pub part: Option<usize>,
}