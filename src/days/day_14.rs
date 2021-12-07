use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let mut found = vec![];
    let mut pending = HashMap::new();

    let mut final_end = usize::MAX - 1;
    let mut i = 0;
    let mut start = 0;
    let mut end = 1000;
    while start <= final_end {
        let new_found = (start..end)
            .into_par_iter()
            .filter_map(|i| {
                let mut s = format!("{}{}", input, i);
                for _ in 0..2017 {
                    s = format!("{:x}", md5::compute(s));
                }
                let mut three_seq = None;
                let mut five_seq = vec![];
                let mut prev = '_';
                let mut cnt = 0;
                s.push('.'); // cnt is only evaluated if c != prev, so make sure that happens at the end
                for c in s.chars() {
                    if c == prev {
                        cnt += 1;
                    } else {
                        if cnt >= 5 {
                            five_seq.push(prev);
                        } else if cnt >= 3 && three_seq.is_none() {
                            three_seq = Some(prev);
                        }
                        cnt = 1;
                        prev = c;
                    }
                }
                if three_seq.is_some() || !five_seq.is_empty() {
                    Some((i, three_seq, five_seq))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for (i, three_seq, five_seq) in new_found {
            if let Some(c) = three_seq {
                pending.entry(c).or_insert_with(Vec::new).push(i);
            }
            for c in &five_seq {
                if let Some(v) = pending.get_mut(c) {
                    for j in v.drain(..) {
                        if (i - j) <= 1000 {
                            found.push(j);
                            if found.len() == 64 {
                                final_end = j + 1000;
                            }
                        }
                    }
                }
            }
        }

        start = end;
        end = (end + 1000).min(final_end);
    }
    found.sort_unstable();
    pv!(found[63]);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let mut found = vec![];
    let mut pending = HashMap::new();

    let mut five_seq = vec![];
    let mut end = usize::MAX - 1;
    let mut i = 0;
    while i < end {
        let hash = md5::compute(format!("{}{}", input, i));
        let hex = format!("{:x}", hash);

        let mut three_seq = None;
        let mut prev = '_';
        let mut cnt = 0;
        five_seq.clear();
        for c in hex.chars().chain(std::iter::once('.')) {
            if c == prev {
                cnt += 1;
            } else {
                if cnt >= 5 {
                    five_seq.push(prev);
                } else if cnt >= 3 && three_seq.is_none() {
                    three_seq = Some(prev);
                }
                cnt = 1;
                prev = c;
            }
        }
        if let Some(c) = three_seq {
            pending.entry(c).or_insert_with(Vec::new).push(i);
        }
        for c in &five_seq {
            if let Some(v) = pending.get_mut(c) {
                for j in v.drain(..) {
                    if (i - j) <= 1000 {
                        found.push(j);
                        if found.len() == 64 {
                            end = j + 1000;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    found.sort_unstable();
    pv!(found[63]);
}
