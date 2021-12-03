// region: Part 1
pub fn part1(inp: String) -> () {
    let num_bits = inp.lines().peekable().peek().unwrap().len();
    let num_lines = inp.lines().count();
    let mut counts = vec![0usize; num_bits];
    inp.lines().for_each(|line| count_bits(&mut counts, line));
    let gamma = counts.iter().map(|&count| {
        if count > (num_lines / 2) {
            '1'
        } else {
            '0'
        }
    }).collect::<String>();
    let epsilon = gamma.chars().map(|c| {
        match c {
            '0' => '1',
            '1' => '0',
            _ => panic!("Encountered unexpected character"),
        }
    }).collect::<String>();
    let gamma =  usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon =  usize::from_str_radix(&epsilon, 2).unwrap();

    let res = gamma * epsilon;
    println!("Result for day 3, part 1: {}", res);
}

fn count_bits(counts: &mut Vec<usize>, line: &str) {
    counts.iter_mut().zip(line.chars()).for_each(|(count, char)|{
        match char {
            '0' => (),
            '1' => *count += 1,
            _ => panic!("Encountered unexpected character"),
        }
    });
}
// endregion

// region: Part 2
pub fn part2(_inp: String) -> () {
    println!{"Hello World!"};
}
// endregion