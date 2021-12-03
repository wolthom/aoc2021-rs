// region: Part 1
pub fn part1(inp: String) -> () {
    let mut pos = Position::new();
    inp.lines().for_each(|l| {
        let pos = &mut pos;
        pos.update(l)
    });
    println!("Result for day 2, part 1: {}", pos.result());
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn update(&mut self, line: &str) {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        assert!(
            split.len() == 2,
            "Splitting on whitespace did not lead to exactly two parts"
        );

        let (dir, dist) = (split[0], split[1].parse::<i64>().unwrap());
        match dir {
            "forward" => self.x += dist,
            "up" => self.y -= dist,
            "down" => self.y += dist,
            _ => panic!("Encountered unexpected direction"),
        };
    }

    fn result(&self) -> i64 {
        self.x * self.y
    }
}

// endregion

// region: Part 2

pub fn part2(_inp: String) -> () {
    println! {"Hello World!"};
}

// endregion
