use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");
    // let input = ;

    let parsed = input
        .split(", ")
        .map(|l| (l.chars().next().unwrap(), parse_u(&l[1..])))
        .to_vec();

    let mut pos = (0, 0);
    let mut dir = Dir::Up;
    let mut seen = HashSet::new();

    for (d, l) in parsed {
        if d == 'L' {
            dir = dir.counter_clockwise();
        } else {
            dir = dir.clockwise();
        }
        for _ in 0..l {
            pos += dir;
            if !seen.insert(pos) {
                pv!(manhattan_i(pos, (0, 0)));
                return;
            }
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");
    // let input = ;

    let parsed = input
        .split(", ")
        .map(|l| (l.chars().next().unwrap(), parse_u(&l[1..])))
        .to_vec();

    let mut pos = (0, 0);
    let mut dir = Dir::Up;

    for (d, l) in parsed {
        if d == 'L' {
            dir = dir.counter_clockwise();
        } else {
            dir = dir.clockwise();
        }
        for _ in 0..l {
            pos += dir;
        }
    }
    pv!(manhattan_i(pos, (0, 0)));
}
