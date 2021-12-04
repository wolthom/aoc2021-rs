const BIT_WIDTH: usize = 12;
const BIT_MASK: usize = 0b1111_1111_1111;

// region: Part 1
pub fn part1(inp: String) -> () {
    // Set up data
    let nums = inp.lines().map(|l| usize::from_str_radix(l, 2).unwrap()).collect::<Vec<_>>();
    let mut counts = vec![0usize; BIT_WIDTH];
    // Update bit counts for each number
    nums.iter().for_each(|num| count_bits(&mut counts, *num));
    // Parse vec into gamma
    let gamma = counts.iter().enumerate().fold(0usize, |acc, (idx, num)| {
        if *num > (nums.len() / 2) {
            acc |  (1 << idx)
        } else {
            acc
        }
    });
    let epsilon = gamma ^ BIT_MASK;
    println!("Result for day 3, part 1: {}", epsilon * gamma);
}
// endregion

// region: Part 2
pub fn part2(inp: String) -> () {
    // Set up data
    let nums = inp.lines().map(|l| usize::from_str_radix(l, 2).unwrap()).collect::<Vec<_>>();
    let mut counts = vec![0usize; BIT_WIDTH];
    // Update bit counts for each number
    nums.iter().for_each(|num| count_bits(&mut counts, *num));
    dbg!(&counts);
}
// endregion

// region: Common functions
fn count_bits(counts: &mut Vec<usize>, num: usize) {
    for idx in 0..BIT_WIDTH {
        let mask = 1usize << idx;
        if (num & mask) > 0 {
            counts[idx] += 1;
        }
    }
}
// endregion 