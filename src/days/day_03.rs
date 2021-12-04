use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let parsed = input
        .lines()
        .map(|l| l.split_whitespace().map(parse).to_vec())
        .to_vec();

    let mut count = 0;
    for block in parsed.chunks(3) {
        for col in 0..3 {
            let mut a = block.iter().map(|r| r[col]).to_vec();
            a.sort_unstable();
            if a[0] + a[1] > a[2] {
                count += 1;
            }
        }
    }
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let parsed = input
        .lines()
        .map(|l| l.split_whitespace().map(parse).to_vec())
        .filter(|a| {
            let mut a = a.clone();
            a.sort_unstable();
            a[0] + a[1] > a[2]
        })
        .count();
    pv!(parsed);
}
