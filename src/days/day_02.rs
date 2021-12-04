use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");
    // let input = ;

    let parsed = input
        .lines()
        .map(|l| l.chars().map(Dir::from).to_vec())
        .to_vec();

    let mut keypad = "  1
 234
56789
 ABC
  D"
    .lines()
    .enumerate()
    .flat_map(|(y, l)| {
        l.chars()
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .map(move |(x, c)| ((x as isize, y as isize), c))
    })
    .to_map();

    let mut pos = *keypad.iter().find(|(_, c)| **c == '5').unwrap().0;
    for l in parsed {
        for d in l {
            let next = pos + d;
            if keypad.contains_key(&next) {
                pos = next;
            }
        }
        print!("{}", keypad[&pos]);
    }
    println!();
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let parsed = input
        .lines()
        .map(|l| l.chars().map(Dir::from).to_vec())
        .to_vec();

    let mut pos = (1isize, 1);
    for l in parsed {
        for d in l {
            let next = pos + d;
            if next.0 < 0 || next.1 < 0 || next.0 > 2 || next.1 > 2 {
                continue;
            }
            pos = next;
        }
        print!("{}", pos.1 * 3 + pos.0 + 1);
    }
    println!();
}
