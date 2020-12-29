use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");
    let parsed = input
        .lines()
        .filter_map(|l| {
            let s = l.split('[').to_vec();
            let check = s[1].strip_suffix(']').unwrap();
            let mut s = s[0].rsplit('-');
            let id = parse(s.next().unwrap());
            let name = s.rev().to_vec();
            let mut frequencies = HashMap::new();
            for c in name.iter().flat_map(|n| n.chars()) {
                *frequencies.entry(c).or_insert(0) += 1;
            }
            let mut frequencies = frequencies.iter().to_vec();
            frequencies.sort_by(|a, b| {
                if a.1 == b.1 {
                    a.0.cmp(&b.0)
                } else {
                    b.1.cmp(&a.1)
                }
            });
            if frequencies
                .iter()
                .zip(check.chars())
                .take(check.len())
                .all(|((c, _), check)| **c == check)
            {
                Some((id, name.join("-")))
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
                        (((c as isize - 97) + id) % 26 + 97) as u8 as char
                    }
                })
                .collect::<String>();
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
        .filter_map(|l| {
            let s = l.split('[').to_vec();
            let check = s[1].strip_suffix(']').unwrap();
            let mut s = s[0].rsplit('-');
            let id = parse(s.next().unwrap());
            let name = s.rev().to_vec();
            let mut frequencies = HashMap::new();
            for c in name.iter().flat_map(|n| n.chars()) {
                *frequencies.entry(c).or_insert(0) += 1;
            }
            let mut frequencies = frequencies.iter().to_vec();
            frequencies.sort_by(|a, b| {
                if a.1 == b.1 {
                    a.0.cmp(&b.0)
                } else {
                    b.1.cmp(&a.1)
                }
            });
            if frequencies
                .iter()
                .zip(check.chars())
                .take(check.len())
                .all(|((c, _), check)| **c == check)
            {
                Some(id)
            } else {
                None
            }
        })
        .sum::<isize>();
    pv!(parsed);
}
