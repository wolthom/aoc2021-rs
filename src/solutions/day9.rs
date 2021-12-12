use smallvec::SmallVec;

pub fn part1(inp: String) -> () {
    let vals = inp
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect::<Vec<_>>();
    let ncols = inp.trim().split('\n').nth(0).unwrap().chars().count() as isize;
    let nrows = inp.trim().chars().filter(|c| *c == '\n').count() as isize + 1;

    let mut buf: SmallVec<[isize; 4]> = SmallVec::new();
    let mut extrema: Vec<isize> = Vec::new();

    for idx in 0..(ncols * nrows) {
        neighbor_idxs(idx, ncols, nrows, &mut buf);
        let cur_val = vals[idx as usize];
        let is_min = buf.drain(..).all(|v| vals[v as usize] > cur_val);
        if is_min {
            extrema.push(cur_val + 1)
        }
    }

    println!(
        "Result for day 9, part 1: {}",
        extrema.iter().sum::<isize>()
    );
}

fn neighbor_idxs(idx: isize, ncols: isize, nrows: isize, buf: &mut SmallVec<[isize; 4]>) {
    let ridx = idx / ncols;
    let cidx = idx % ncols;

    let candidates = [
        (ridx - 1, cidx),
        (ridx, cidx - 1),
        (ridx, cidx + 1),
        (ridx + 1, cidx),
    ];

    candidates
        .into_iter()
        .filter(|cand| idx_is_valid(*cand, ncols, nrows))
        .for_each(|cand| buf.push(cand.0 * ncols + cand.1));
}

fn idx_is_valid((ridx, cidx): (isize, isize), ncols: isize, nrows: isize) -> bool {
    (ridx >= 0) && (ridx < nrows) && (cidx >= 0) && (cidx < ncols)
}

pub fn part2(_inp: String) -> () {
    println!("Result for day 9, part 2: ");
}
