use std::collections::BTreeMap;
use lazy_static::lazy_static;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

type Callback = fn(data: String) -> ();

lazy_static!{
    pub static ref SOLS: BTreeMap<(usize, usize), Callback>= {
        let mut m = BTreeMap::new();
        m.insert((1,1),  day1::part1  as Callback);
        m.insert((1,2),  day1::part2  as Callback);
        m.insert((2,1),  day2::part1  as Callback);
        m.insert((2,2),  day2::part2  as Callback);
        m.insert((3,1),  day3::part1  as Callback);
        m.insert((3,2),  day3::part2  as Callback);
        m.insert((4,1),  day4::part1  as Callback);
        m.insert((4,2),  day4::part2  as Callback);
        m.insert((5,1),  day5::part1  as Callback);
        m.insert((5,2),  day5::part2  as Callback);
        m.insert((6,1),  day6::part1  as Callback);
        m.insert((6,2),  day6::part2  as Callback);
        m.insert((7,1),  day7::part1  as Callback);
        m.insert((7,2),  day7::part2  as Callback);
        m.insert((8,1),  day8::part1  as Callback);
        m.insert((8,2),  day8::part2  as Callback);
        m.insert((9,1),  day9::part1  as Callback);
        m.insert((9,2),  day9::part2  as Callback);
        m
    };
}