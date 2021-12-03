use std::collections::BTreeMap;
use lazy_static::lazy_static;

mod day1;
mod day2;
mod day3;


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
        m
    };
}