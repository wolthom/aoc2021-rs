const BIT_WIDTH: usize = 12;
const BIT_MASK: usize = 0b1111_1111_1111;
// const BIT_WIDTH: usize = 5;
// const BIT_MASK: usize = 0b11111;


// region: Part 1
pub fn part1(inp: String) -> () {
    // Set up data
    let nums = inp.lines().map(|l| usize::from_str_radix(l, 2).unwrap()).collect::<Vec<_>>();
    let counts = count_bits(&nums);

    // Parse vec into gamma
    let gamma = counts.iter().enumerate().fold(0usize, |acc, (idx, num)| {
        let shift = BIT_WIDTH - idx - 1;
        if *num > (nums.len() / 2) {
            acc |  (1 << shift)
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

    let ox_val = select_val(&nums, false);

    let co2_val = select_val(&nums, true);
    println!("Result for day 3, part 1: {}", ox_val * co2_val);
}

fn predicate(num: usize, pos: usize, bit: usize) -> bool {
    let shift = BIT_WIDTH - pos - 1;
    ((num & 1 << shift) >> shift ) == bit
}

fn select_val(nums: &[usize], co2_pred: bool) -> usize {
    let mut selection = nums.to_owned();
    let mut idx = 0usize;
    let mut counts = count_bits(&selection);
    while selection.len() > 1 {
        let n = selection.len() / 2 + selection.len() % 2;
        let mut bit = if counts[idx] >= n {
            1usize
        } else {
            0usize
        };
        if co2_pred {
            bit = bit ^ 1;
        }
        selection = selection.into_iter().filter(|num| predicate(*num, idx, bit)).collect();
        counts = count_bits(&selection);
        idx += 1;
    }
    selection[0]
}
// endregion

// region: Common functions
fn count_bits(nums: &[usize]) -> Vec<usize> {
    let mut out = vec![0; BIT_WIDTH];
    for num in nums{
        for idx in 0..BIT_WIDTH {
            let shift = BIT_WIDTH - idx - 1;
            let mask = 1usize << shift;
            if (num & mask) > 0 {
                out[idx] += 1;
            }
        }        
    }
    out
}
// endregion 