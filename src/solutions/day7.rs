pub fn part1(inp: String) -> () {
    let crabs: Vec<i64> = inp.trim().split(',').map(|p| i64::from_str_radix(p, 10).unwrap()).collect();
    let xmin = crabs.iter().min().unwrap().clone();
    let xmax = crabs.iter().max().unwrap().clone();
    let r = xmin..=xmax;

    let costs = r.map(|pos| {
        crabs.iter().fold(0, |acc, val| {
            acc + linear_cost(pos, *val)
        })
    }).collect::<Vec<_>>();

    println!("Result for day 7, part 1: {}", costs.iter().min().unwrap());
}

fn linear_cost(target: i64, cur: i64) -> i64 {
    (target - cur).abs()
}


pub fn part2(inp: String) -> () {
    let crabs: Vec<i64> = inp.trim().split(',').map(|p| i64::from_str_radix(p, 10).unwrap()).collect();
    let xmin = crabs.iter().min().unwrap().clone();
    let xmax = crabs.iter().max().unwrap().clone();
    let r = xmin..=xmax;

    let costs = r.map(|pos| {
        crabs.iter().fold(0, |acc, val| {
            acc +superlinear_cost(pos, *val)
        })
    }).collect::<Vec<_>>();

    println!("Result for day 7, part 2: {}", costs.iter().min().unwrap());
}

fn superlinear_cost(target: i64, cur: i64) -> i64 {
    let delta = (target - cur).abs();
    // Sum of integers up to n: n(n + 1)/2
    delta * (delta + 1) / 2
}
