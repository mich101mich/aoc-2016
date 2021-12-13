use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let grid = hashtag_grid(input);
    let pos_to_number = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| ('0'..='9').contains(c))
                .map(move |(x, c)| ((x, y), parse_c(c)))
        })
        .to_map();
    let n = pos_to_number.len();
    let mut number_to_pos = vec![(0, 0); n];
    for (pos, n) in pos_to_number.iter() {
        number_to_pos[*n] = *pos;
    }

    let mut edges = vec![vec![usize::MAX; n]; n];

    let mut goals = number_to_pos.clone();
    for (a, pos) in number_to_pos.iter().enumerate().rev() {
        let paths = grid.dijkstra(*pos, &goals, |b| !*b);
        for (goal, path) in paths {
            let b = pos_to_number[&goal];
            if a != b {
                edges[a][b] = path.cost;
                edges[b][a] = path.cost;
            }
        }
        goals.pop();
    }

    let found_all = (1 << n) - 1u16;
    let path = a_star_search(
        |(pos, found), out| {
            if found == found_all {
                out.push(((n, found_all), edges[pos][0]));
                return;
            }
            out.extend(
                edges[pos]
                    .iter()
                    .enumerate()
                    .filter(|(x, _)| found & (1 << x) == 0)
                    .filter(|(x, cost)| **cost != usize::MAX)
                    .map(|(x, cost)| ((x, found | (1 << x)), *cost)),
            );
        },
        (0, 1),
        (n, found_all),
        |(_, found)| n - found.count_ones() as usize,
    );
    pv!(path.unwrap().cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let grid = hashtag_grid(input);
    let pos_to_number = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| ('0'..='9').contains(c))
                .map(move |(x, c)| ((x, y), parse_c(c)))
        })
        .to_map();
    let n = pos_to_number.len();
    let mut number_to_pos = vec![(0, 0); n];
    for (pos, n) in pos_to_number.iter() {
        number_to_pos[*n] = *pos;
    }

    let mut edges = vec![vec![usize::MAX; n]; n];

    let mut goals = number_to_pos.clone();
    for (a, pos) in number_to_pos.iter().enumerate().rev() {
        let paths = grid.dijkstra(*pos, &goals, |b| !*b);
        for (goal, path) in paths {
            let b = pos_to_number[&goal];
            if a != b {
                edges[a][b] = path.cost;
                edges[b][a] = path.cost;
            }
        }
        goals.pop();
    }

    let found_all = (1 << n) - 1u16;
    let path = a_star_search(
        |(pos, found), out| {
            if found == found_all {
                out.push(((n, found_all), 0));
                return;
            }
            out.extend(
                edges[pos]
                    .iter()
                    .enumerate()
                    .filter(|(x, _)| found & (1 << x) == 0)
                    .filter(|(x, cost)| **cost != usize::MAX)
                    .map(|(x, cost)| ((x, found | (1 << x)), *cost)),
            );
        },
        (0, 1),
        (n, found_all),
        |(_, found)| n - found.count_ones() as usize,
    );
    pv!(path.unwrap().cost);
}
