fn inp_to_vec(inp: &str) -> Vec<usize> {
    inp
    .split_whitespace()
    .map(|el| el.parse::<usize>())
    .collect::<Result<Vec<_>, _>>()
    .expect("Input could not be parsed into integer")
}

pub fn part1(inp: String) -> () {
    // Turn string of lines into vector of integers
    let vals = inp_to_vec(&inp);
    
    let res = vals.windows(2).fold(0, |acc, win| {
        if win[1] > win[0] {
            acc + 1
        } else {
            acc
        }
    });

    println!("Result for day 1, part 1: {}", res);
}

pub fn part2(inp: String) -> () {
    // Turn string of lines into vector of integers
    let vals = inp_to_vec(&inp);

    let res = vals.windows(4).fold(0, |acc, win| {
        if win[3] > win[0] {
            acc + 1
        } else {
            acc
        }
    });

    println!("Result for day 1, part 2: {}", res);
}