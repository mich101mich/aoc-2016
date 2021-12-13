use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");
    let mut parsed = input
        .lines()
        .map(|l| scanf!(l, "{}-{}", usize, usize).unwrap())
        .map(|(a, b)| (a..=b))
        .to_vec();

    let mut ranges = vec![0..=4294967295usize];

    let mut earliest = 0;
    for x in parsed {
        let mut new_ranges = vec![];
        for range in ranges {
            if x.end() < range.start() || x.start() > range.end() {
                // no overlap
                new_ranges.push(range);
            } else if x.start() <= range.start() && x.end() >= range.end() {
                // x      |------|
                // range    |--|
                continue;
            } else if x.start() <= range.start() {
                // x     |----|
                // range    |----|
                new_ranges.push(*x.end() + 1..=*range.end());
            } else if x.end() >= range.end() {
                // x        |----|
                // range  |----|
                new_ranges.push(*range.start()..=*x.start() - 1);
            } else {
                // x        |---|
                // range  |-------|
                new_ranges.push(*range.start()..=*x.start() - 1);
                new_ranges.push(*x.end() + 1..=*range.end());
            }
        }
        ranges = new_ranges;
        ranges.retain(|r| !r.is_empty());
    }
    let count = ranges
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<usize>();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");
    let mut parsed = input
        .lines()
        .map(|l| scanf!(l, "{}-{}", usize, usize).unwrap())
        .to_vec();

    parsed.sort_unstable();

    let mut earliest = 0;
    for (a, b) in parsed {
        if a <= earliest {
            earliest = earliest.max(b + 1);
        }
    }
    pv!(earliest);
}
