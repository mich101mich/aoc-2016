use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let mut parsed = input
        .lines()
        .map(|l| {
            scanf!(
                l,
                "Disc #{} has {} positions; at time=0, it is at position {}.",
                usize,
                usize,
                usize
            )
            .unwrap()
        })
        .map(|(_, n, start)| (n, start))
        .to_vec();

    parsed.push((11, 0));

    let start_time = (0..)
        .find(|&t| {
            parsed
                .iter()
                .enumerate()
                .all(|(offset, (n, start_pos))| (start_pos + t + offset + 1) % n == 0)
        })
        .unwrap();
    pv!(start_time);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let parsed = input
        .lines()
        .map(|l| {
            scanf!(
                l,
                "Disc #{} has {} positions; at time=0, it is at position {}.",
                usize,
                usize,
                usize
            )
            .unwrap()
        })
        .map(|(_, n, start)| (n, start))
        .to_vec();

    let start_time = (0..)
        .find(|&t| {
            parsed
                .iter()
                .enumerate()
                .all(|(offset, (n, start_pos))| (start_pos + t + offset + 1) % n == 0)
        })
        .unwrap();
    pv!(start_time);
}
