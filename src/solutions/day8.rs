use std::collections::{BTreeMap, BTreeSet};

pub fn part1(inp: String) -> () {
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

pub fn part2(inp: String) -> () {
    let readings = inp
        .trim()
        .split('\n')
        .map(|l| parse_digit(l))
        .collect::<Vec<_>>();

    println!("Result for day 8, part 2: {}", readings.iter().sum::<u32>());
}

type Pattern = BTreeSet<char>;
type PatternMap = BTreeMap<u8, Pattern>;

fn parse_digit(l: &str) -> u32 {
    let mut parts = l.split(" | ");
    let mut pattern_map: PatternMap = BTreeMap::new();

    let mut patterns = parts
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.chars().collect::<Pattern>())
        .collect::<Vec<_>>();

    pattern_map.insert(1, find_len(&mut patterns, 2)); // 1
    pattern_map.insert(7, find_len(&mut patterns, 3)); // 7
    pattern_map.insert(4, find_len(&mut patterns, 4)); // 4
    pattern_map.insert(8, find_len(&mut patterns, 7)); // 8
    pattern_map.insert(9, find_overlap(&mut patterns, &pattern_map[&4], 6, 4)); // 9
    pattern_map.insert(0, find_overlap(&mut patterns, &pattern_map[&1], 6, 2)); // 0
    pattern_map.insert(6, find_len(&mut patterns, 6)); // 6
    pattern_map.insert(3, find_overlap(&mut patterns, &pattern_map[&1], 5, 2)); // 2
    pattern_map.insert(5, find_overlap(&mut patterns, &pattern_map[&6], 5, 5)); // 5
    pattern_map.insert(2, patterns.pop().unwrap()); // 3

    let digits = parts
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.chars().collect::<BTreeSet<_>>())
        .map(|pat| {
            let (digit, _) = pattern_map.iter().find(|(_, v)| pat == **v).unwrap();
            *digit
        })
        .collect::<Vec<_>>();

    digits_to_num(&digits)
}

fn digits_to_num(digits: &Vec<u8>) -> u32 {
    let mut out = 0;
    for (idx, digit) in digits.iter().rev().enumerate() {
        out += (*digit as u32) * 10u32.pow(idx.try_into().unwrap());
    }
    out
}

fn find_len(patterns: &mut Vec<Pattern>, pat_len: usize) -> Pattern {
    let idx = patterns.iter().position(|p| p.len() == pat_len).unwrap();
    patterns.swap_remove(idx)
}

fn find_overlap(
    patterns: &mut Vec<Pattern>,
    find_pat: &Pattern,
    pat_len: usize,
    overlap: usize,
) -> Pattern {
    let mut candidates = patterns
        .iter()
        .enumerate()
        .filter(|(_, pat)| pat.len() == pat_len);

    let (idx, _) = candidates
        .find(|(_, pat)| pat.intersection(find_pat).count() == overlap)
        .unwrap();

    patterns.swap_remove(idx)
}
