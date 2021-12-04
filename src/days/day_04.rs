use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");
    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{}-{}[{}]", String, usize, String).unwrap())
        .filter_map(|(name, id, check)| {
            let mut counts = HashMap::new();
            for c in name.chars().filter(|c| *c != '-') {
                *counts.entry(c).or_insert(0) += 1;
            }
            let mut counts = counts.into_iter().to_vec();
            counts.sort_by(|a, b| a.1.cmp(&b.1).reverse().then(a.0.cmp(&b.0)));

            if counts
                .iter()
                .zip(check.chars())
                .all(|((c, _), check)| *c == check)
            {
                Some((id, name))
            } else {
                None
            }
        })
        .map(|(id, name)| {
            let decoded = name
                .chars()
                .map(|c| {
                    if c == '-' {
                        ' '
                    } else {
                        let mut n = c as usize - b'a' as usize;
                        n = (n + id) % 26;
                        (n as u8 + b'a') as char
                    }
                })
                .to_string();
            (id, decoded)
        })
        .filter(|(_, name)| name.contains("pole"))
        .for_each(|o| pv!(o));
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");
    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{}-{}[{}]", String, usize, String).unwrap())
        .filter_map(|(name, id, check)| {
            let mut counts = HashMap::new();
            for c in name.chars().filter(|c| *c != '-') {
                *counts.entry(c).or_insert(0) += 1;
            }
            let mut counts = counts.into_iter().to_vec();
            counts.sort_by(|a, b| a.1.cmp(&b.1).reverse().then(a.0.cmp(&b.0)));

            if counts
                .iter()
                .zip(check.chars())
                .all(|((c, _), check)| *c == check)
            {
                Some(id)
            } else {
                None
            }
        })
        .sum::<usize>();
    pv!(parsed);
}
