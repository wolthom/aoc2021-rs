use std::{collections::HashMap, num::ParseIntError, str::FromStr};

pub fn part1(inp: String) -> () {
    let lines = inp
        .trim()
        .split('\n')
        .map(|l| Line::from_str(l).unwrap())
        .collect::<Vec<_>>();

    let mut buf: Vec<Pos> = Vec::new();
    let mut marks: HashMap<Pos, usize> = HashMap::new();
    for l in lines.iter() {
        l.push_marks(&mut buf, false);
        for p in buf.drain(..) {
            let e = marks.entry(p).or_insert(0);
            *e += 1;
        }
    }

    let mut count = 0usize;
    for v in marks.values() {
        if *v >= 2 {
            count += 1;
        }
    }
    println!("Result for day 5, part 1: {}", count);
}

pub fn part2(inp: String) -> () {
    let lines = inp
        .trim()
        .split('\n')
        .map(|l| Line::from_str(l).unwrap())
        .collect::<Vec<_>>();

    let mut buf: Vec<Pos> = Vec::new();
    let mut marks: HashMap<Pos, usize> = HashMap::new();
    for l in lines.iter() {
        l.push_marks(&mut buf, true);
        for p in buf.drain(..) {
            let e = marks.entry(p).or_insert(0);
            *e += 1;
        }
    }

    let mut count = 0usize;
    for v in marks.values() {
        if *v >= 2 {
            count += 1;
        }
    }
    println!("Result for day 5, part 2: {}", count);
}

// region: Utility types
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl FromStr for Pos {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let (x, y) = (parts.next().unwrap(), parts.next().unwrap());
        let (x, y) = (i64::from_str_radix(x, 10)?, i64::from_str_radix(y, 10)?);
        Ok(Self::new(x, y))
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Line {
    start: Pos,
    end: Pos,
}

impl Line {
    fn new(start: Pos, end: Pos) -> Self {
        Self { start, end }
    }

    fn push_marks(&self, buf: &mut Vec<Pos>, diag: bool) {
        self.push_marks_h(buf);
        self.push_marks_v(buf);
        if diag {
            self.push_marks_d(buf);
        }
    }

    fn push_marks_h(&self, buf: &mut Vec<Pos>) {
        // Return if not horizontal
        if self.start.y != self.end.y {
            return;
        }
        // Iterate over x-values
        let r = self.start.x.min(self.end.x)..=self.start.x.max(self.end.x);
        for x in r {
            buf.push(Pos::new(x, self.start.y));
        }
    }

    fn push_marks_v(&self, buf: &mut Vec<Pos>) {
        // Return if not vertical
        if self.start.x != self.end.x {
            return;
        }
        // Iterate over y-values
        let r = self.start.y.min(self.end.y)..=self.start.y.max(self.end.y);
        for y in r {
            buf.push(Pos::new(self.start.x, y));
        }
    }

    fn push_marks_d(&self, buf: &mut Vec<Pos>) {
        let diff = ((self.end.x - self.start.x), (self.end.y - self.start.y));
        // Return if not 45° diagonal
        if diff.0.abs() != diff.1.abs() {
            return;
        }
        let delta = (diff.0.signum(), diff.1.signum());
        let mut cur = self.start.clone();
        while cur != self.end {
            buf.push(cur.clone());
            cur = Pos::new(cur.x + delta.0, cur.y + delta.1);
        }
        buf.push(self.end.clone());
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        let (start, end) = (parts.next().unwrap(), parts.next().unwrap());

        Ok(Self::new(Pos::from_str(start)?, Pos::from_str(end)?))
    }
}
// endregion
