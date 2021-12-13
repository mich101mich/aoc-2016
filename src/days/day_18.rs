use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let parsed = dotted_line(input, '^');
    let n = parsed.len();

    let rows = 400000;
    let mut row = parsed;
    let mut next_row = row.clone();
    let mut count = row.iter().filter(|c| !*c).count();

    for _ in 1..rows {
        let left = std::iter::once(&false).chain(row.iter());
        let right = row.iter().skip(1).chain(std::iter::once(&false));
        next_row
            .iter_mut()
            .zip(left.zip(right))
            .for_each(|(next, (left, right))| {
                *next = *left != *right;
                if !*next {
                    count += 1;
                }
            });
        std::mem::swap(&mut row, &mut next_row);
    }

    pv!(count);
    // 19986747 high
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let parsed = dotted_line(input, '^');
    let n = parsed.len();

    let rows = 40;
    let mut grid = Grid::new_clone((n, rows), false);
    grid.row_mut(0)
        .zip(parsed.iter())
        .for_each(|(cell, c)| *cell = *c);

    for i in 1..rows {
        let (prev, cur) = grid.split_at_mut(i);
        let prev = prev.last().unwrap();
        let cur = &mut cur[0];

        let first_window = [false, prev[0], prev[1]];
        let last_window = [prev[n - 2], prev[n - 1], false];
        let prev_iter = std::iter::once(&first_window[..])
            .chain(prev.windows(3))
            .chain(std::iter::once(&last_window[..]));

        for (cur, prev) in cur.iter_mut().zip(prev_iter) {
            *cur = prev[0] != prev[2];
        }
    }

    let count = grid.grid_iter().filter(|&c| !*c).count();
    pv!(count);
}
