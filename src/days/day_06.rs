use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let parsed = input.lines().to_vec();

    let mut counts = vec![HashMap::new(); parsed[0].len()];

    for line in parsed {
        for (i, c) in line.chars().enumerate() {
            *counts[i].entry(c).or_insert(0) += 1;
        }
    }
    let res = counts
        .into_iter()
        .map(|col| col.into_iter().min_by_key(|(_, v)| *v).unwrap().0)
        .to_string();
    pv!(res);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let parsed = input.lines().to_vec();

    let mut counts = vec![HashMap::new(); parsed[0].len()];

    for line in parsed {
        for (i, c) in line.chars().enumerate() {
            *counts[i].entry(c).or_insert(0) += 1;
        }
    }
    let res = counts
        .into_iter()
        .map(|col| col.into_iter().max_by_key(|(_, v)| *v).unwrap().0)
        .to_string();
    pv!(res);
}
