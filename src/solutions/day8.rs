pub fn part1(inp: String) -> () {
    // 1, 4, 7, 8
    let lens = inp
        .trim()
        .lines()
        .map(|l| l.split(" | ").nth(1)) // Lines to noted value
        .map(|p| p.unwrap().split(' ')) // Noted value to individual digits
        .flatten()
        .map(|s| s.len()); // Number of active segments for each digit

    let mut res = 0;
    for l in lens {
        if [2, 3, 4, 7].contains(&(l as i32)) {
            res += 1;
        }
    }
    println!("Result for day 8, part 1: {}", res);
}

pub fn part2(_inp: String) -> () {
    println!("Result for day 8, part 2: ");
}
