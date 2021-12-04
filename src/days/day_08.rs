use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let (w, h) = (50, 6);
    let mut screen = Grid::new_clone((w, h), false);

    let parsed = input.lines().for_each(|l| {
        if let Some((w, h)) = scanf!(l, "rect {}x{}", usize, usize) {
            screen.fill_rect((0, 0), (w, h), true);
        } else if let Some((y, offset)) = scanf!(l, "rotate row y={} by {}", usize, usize) {
            screen[y].rotate_right(offset % w);
        } else if let Some((x, offset)) = scanf!(l, "rotate column x={} by {}", usize, usize) {
            let col = screen.col(x).copied().to_vec();
            let iter = screen
                .col_mut(x)
                .zip(col.iter().cycle().skip(h - offset % h));
            for (a, b) in iter {
                *a = *b;
            }
        } else {
            panic!("Unrecognized input: {}", l);
        }
    });
    screen.print('#', ' ');
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let (w, h) = (50, 6);
    let mut screen = Grid::new_clone((w, h), false);

    let parsed = input.lines().for_each(|l| {
        if let Some((w, h)) = scanf!(l, "rect {}x{}", usize, usize) {
            screen.fill_rect((0, 0), (w, h), true);
        } else if let Some((y, offset)) = scanf!(l, "rotate row y={} by {}", usize, usize) {
            screen[y].rotate_right(offset % w);
        } else if let Some((x, offset)) = scanf!(l, "rotate column x={} by {}", usize, usize) {
            let col = screen.col(x).copied().to_vec();
            let iter = screen
                .col_mut(x)
                .zip(col.iter().cycle().skip(h - offset % h));
            for (a, b) in iter {
                *a = *b;
            }
        } else {
            panic!("Unrecognized input: {}", l);
        }
    });
    let res = screen.grid_iter().filter(|&b| *b).count();
    pv!(res);
}
