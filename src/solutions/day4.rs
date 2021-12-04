use std::{str::FromStr, num::ParseIntError};

const ROWS: usize = 5;
const COLS: usize = 5;

struct Board{
    nums: [u8; ROWS*COLS],
    marks: Vec<(u8, u8)>,
}

impl Board{
    fn new() -> Self {
        Self {
            nums: [0; ROWS*COLS],
            marks: Vec::new(),
        }
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b = Board::new();
        s.split_whitespace()
        .map(|num_str| u8::from_str_radix(num_str, 10).unwrap())
        .enumerate()
        .for_each(|(idx, val)| b.nums[idx] = val);
        Ok(b)
    }
}

pub fn part1(inp: String) -> () {
    println!("Result for day 4, part 1: ");
}



pub fn part2(inp: String) -> () {
    println!("Result for day 4, part 2: ");
}
