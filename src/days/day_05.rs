use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut result = vec!['@'; 8];
    let mut found = 0;

    for i in 0.. {
        let hash = md5::compute(format!("{}{}", input, i));
        let hex = format!("{:x}", hash);
        if let Some(res) = hex.strip_prefix("00000") {
            let index = res.chars().next().unwrap();
            if index < '0' || index > '7' {
                continue;
            }
            let index = index as usize - '0' as usize;
            if result[index] != '@' {
                continue;
            }
            let c = res.chars().nth(1).unwrap();
            result[index] = c;
            print_arr!(result);

            found += 1;
            if found == 8 {
                break;
            }
        }
    }
    println!();
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut found = 0;

    for i in 0.. {
        let hash = md5::compute(format!("{}{}", input, i));
        let hex = format!("{:x}", hash);
        if let Some(res) = hex.strip_prefix("00000") {
            print!("{}", res.chars().next().unwrap());
            found += 1;
            if found == 8 {
                break;
            }
        }
    }
    println!();
}
