use std::collections::BTreeMap;
use std::fmt;
use std::{str::FromStr, num::ParseIntError, fmt::{Debug, Formatter}};

pub fn part1(inp: String) -> () {
    let mut school = School::from_str(&inp).unwrap();
    let num_days = 80;
    for _ in 0..num_days {
        school.step();
    }
    println!("Result for day 6, part 1: {}", school.count());
}
pub fn part2(inp: String) -> () {
    let mut school = School::from_str(&inp).unwrap();
    let num_days = 256;
    for _ in 0..num_days {
        school.step();
    }
    println!("Result for day 6, part 2: {}", school.count());
}

struct School {
    age_counts: BTreeMap<usize, usize>,
}

impl School {
    fn new() -> Self {
        // Create empty map
        let mut map = BTreeMap::new();
        // Insert ages 0 to 8 (inclusive)
        for i in 0..=8 {
            let _ = map.entry(i).or_insert(0);
        }
        Self {
            age_counts: map,
        }
    }

    fn step(&mut self) {
        // Process existing fish and track new offspring
        let mut num_new = 0;
        let mut prev = 0;
        for f in self.age_counts.iter_mut().rev() {
            if (*f.0) == 0 {
                num_new = *f.1;
            }
            let tmp = *f.1;
            *f.1 = prev;
            prev = tmp;
        }
        assert!(true);
        // Add fish that got offspring to 6
        *self.age_counts.get_mut(&6).unwrap() += prev;
        // Add new offspring
        *self.age_counts.get_mut(&8).unwrap() += num_new;
        assert!(true);
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for v in self.age_counts.values(){
            count += *v;
        }
        return count;
    }
}

impl Debug for School {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for fish in &self.age_counts {
            let _ = write!(f, "(Age: {}, Count: {})  ", fish.0, fish.1);
        }
        let _ = write!(f, "\n");
        Ok(())
    }
}

impl FromStr for School {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut school = Self::new();
        let parts = s.trim().split(',');
        for p in parts {
            let age = usize::from_str_radix(p, 10)?;
            *school.age_counts.get_mut(&age).unwrap() += 1;
        }
        Ok(school)
    }
}