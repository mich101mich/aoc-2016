use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let parsed = parse_u(input);
    let mut left = (1..=parsed / 2).collect::<VecDeque<_>>();
    let mut right = (parsed / 2 + 1..=parsed).collect::<VecDeque<_>>();

    while left.len() + right.len() > 1 {
        let taker = left.pop_front().unwrap();
        right.push_back(taker);
        right.pop_front();
        if left.len() + 2 <= right.len() {
            left.push_back(right.pop_front().unwrap());
        }
    }
    if !left.is_empty() {
        pv!(left[0]);
    } else {
        pv!(right[0]);
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let parsed = parse_u(input);
    let mut alive = (1..=parsed).step_by(2).collect::<VecDeque<_>>();
    if parsed % 2 == 1 {
        alive.pop_front();
    }

    while alive.len() > 1 {
        let taker = alive.pop_front().unwrap();
        alive.push_back(taker);
        alive.pop_front();
    }
    pv!(alive[0]);
}
