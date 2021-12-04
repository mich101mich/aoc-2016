use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut result = vec!['_'; 8];
    let mut found = 0;

    let mut start = 0;
    let mut end = 100_000;
    while found < 8 {
        let mut new_found = (start..end)
            .into_par_iter()
            .map(|i| {
                let hash = md5::compute(format!("{}{}", input, i));
                let hex = format!("{:x}", hash);
                (i, hex)
            })
            .filter_map(|(i, hex)| {
                let mut iter = hex.strip_prefix("00000")?.chars();
                let idx = (iter.next()?).to_digit(10)? as usize;
                let c = iter.next()?;
                if idx < 8 {
                    Some((idx, c))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for (idx, c) in new_found {
            if result[idx] == '_' {
                result[idx] = c;
                found += 1;
                print_arr!(result);
            }
        }

        start = end;
        end += 100_000;
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut found = vec![];
    let mut start = 0;
    let mut end = 10_000;
    while found.len() < 8 {
        let new_found = (start..end)
            .into_par_iter()
            .map(|i| {
                let hash = md5::compute(format!("{}{}", input, i));
                let hex = format!("{:x}", hash);
                (i, hex)
            })
            .filter(|(_, hex)| hex.starts_with("00000"))
            .collect::<Vec<_>>();
        found.extend(new_found);

        start = end;
        end += 10_000;
    }
    pv!(found);
    found.truncate(8);
    for (_, s) in found {
        print!("{}", s.chars().nth(5).unwrap());
    }
    println!();
}
