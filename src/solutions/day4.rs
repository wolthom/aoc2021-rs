use std::fmt::{self, Debug};
use std::{str::FromStr, num::ParseIntError, fmt::{Display, Formatter}};

const ROWS: usize = 5;
const COLS: usize = 5;


struct Board{
    nums: [(u8, bool); ROWS*COLS],
    last: u8,
}

impl Board{
    fn new() -> Self {
        Self {
            nums: [(0, false); ROWS*COLS],
            last: 0,
        }
    }

    fn mark_val(&mut self, val: u8) -> bool {
        let mut new_hit = false;
        for el in &mut self.nums {
            if el.0 == val {
                if el.1 {
                    return new_hit;
                }
                self.last = val;
                el.1 = true;
                new_hit = true;
            }
        }
        new_hit
    }

    fn check_row(&self, row: usize) -> bool {
        let mut all_checked = true;
        for col in 0..COLS {
            all_checked &= self.nums[5*row + col].1;
        }
        all_checked
    }

    fn check_col(&self, col: usize) -> bool {
        let mut all_checked = true;
        for row in 0..ROWS {
            all_checked &= self.nums[5*row + col].1;
        }
        all_checked
    }

    fn check(&self) -> bool {
        let mut done = false;
        for ridx in 0..ROWS{
            done = self.check_row(ridx);
            if done {
                return done;
            }
        }
        for cidx in 0..COLS{
            done = self.check_col(cidx);
            if done {
                return done;
            }
        }
        done
    }

    fn score(&self) -> usize {
        let sum: usize = self.nums.iter().filter(|el| !el.1).map(|el| el.0 as usize).sum();
        sum * (self.last as usize)
    }
}


pub fn part1(inp: String) -> () {
    let (vals, boards) = parse_input(&inp);
    let mut boards = boards.map(|board_str| Board::from_str(board_str).unwrap()).collect::<Vec<_>>();
    // dbg!(&boards[9]);
    'outer: for val in &vals {
        // Update each board for current value and check for completion
        for (idx, b) in (&mut boards).iter_mut().enumerate() {
            let hit = b.mark_val(*val);
            if hit {
                let done = b.check();
                if done {
                    println!("Board {} is completed! Score: {}", idx, b.score());
                    break 'outer;
                }
            }
        }
    }
    // dbg!(&boards[9]);
    println!("Result for day 4, part 1: ");
}

fn parse_input(inp: &str) -> (Vec<u8>, impl Iterator<Item = &str>) {
    let mut parts = inp.split("\n\n");
    let vals = parts.next().unwrap();
    let vals = vals.split(',').map(|num_str| u8::from_str_radix(num_str, 10).unwrap()).collect::<Vec<_>>();
    (vals, parts)
}

pub fn part2(inp: String) -> () {
    println!("Result for day 4, part 2: ");
}

// region: Utility impls
impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b = Board::new();
        s.split_whitespace()
        .map(|num_str| u8::from_str_radix(num_str, 10).unwrap())
        .enumerate()
        .for_each(|(idx, val)| b.nums[idx].0 = val);
        Ok(b)
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", *self)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for ridx in 0..ROWS {
            for cidx in 0..COLS{
                let cur = self.nums[5*ridx + cidx];
                write!(f, "{:>2}", cur.0)?;
                if cur.1 {
                    write!(f, "[x]  ")?;
                } else {
                    write!(f, "[ ]  ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}
// endregion