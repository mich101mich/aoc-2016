use crate::utils::*;

#[derive(Debug, Clone, Default)]
struct Node {
    size: usize,
    used: usize,
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let parsed = input
        .lines()
        .skip(2)
        .map(|l| {
            #[rustfmt::skip]
            let ret = scanf_unescaped!(l, r"/dev/grid/node-x{}-y{}\s+{}T\s+{}T\s+{}T\s+{}%", usize, usize, usize, usize, usize, usize).unwrap();
            ret
        })
        .inspect(|(x, y, size, used, avail, use_pct)| {
            assert_eq!(used + avail, *size);
            assert_eq!(used * 100 / size, *use_pct);
        }).to_vec();

    let (w, h) = parsed.iter().fold((0, 0), |(mw, mh), (x, y, ..)| {
        (mw.max(*x + 1), mh.max(*y + 1))
    });

    let mut grid = Grid::new_default((w, h));
    let neighborhood = grid.manhattan();

    for (x, y, size, used, ..) in parsed {
        grid[(x, y)] = Node { size, used }
    }

    let mut target_pos = (w - 1, 0);
    let target_amount = grid[target_pos].used;
    for node in grid.row(0) {
        assert!(node.size >= target_amount);
    }

    let mut clean_blacklist = (0..w).map(|x| (x, 0)).to_set();

    for (pos, v) in grid.grid_iter_index() {
        let mut size_enough = 0;
        let mut remaining_enough = 0;
        for p in neighborhood.get_all_neighbors(pos) {
            let node = &grid[p];
            if v.used <= node.size {
                size_enough += 1;
                if v.used != 0 && node.used != 0 && node.used + v.used <= node.size {
                    remaining_enough += 1;
                }
            }
        }
        // there shouldn't be any nodes that can be directly cleaned
        assert_eq!(remaining_enough, 0);

        // node cannot be cleaned
        if size_enough == 0 {
            clean_blacklist.insert(pos);
        }
    }

    let mut empty_cell = grid.grid_iter_index().find(|(_, v)| v.used == 0).unwrap().0;

    let mut step_count = 0;
    while target_pos != (0, 0) {
        let mut next_pos = target_pos;
        next_pos.0 -= 1;

        let path = a_star_search(
            |pos, out| {
                let amount = grid[pos].used;
                out.extend(
                    neighborhood
                        .get_all_neighbors(pos)
                        .filter(|p| !clean_blacklist.contains(p))
                        .filter(|p| amount <= grid[p].size),
                );
            },
            next_pos,
            empty_cell,
            |p| neighborhood.heuristic(p, empty_cell),
        )
        .unwrap();

        step_count += path.cost;
        for (a, b) in path.path.iter().rev().zip(path.path.iter().rev().skip(1)) {
            grid[*a].used += grid[*b].used;
            grid[*b].used = 0;
        }

        assert_eq!(grid[next_pos].used, 0);
        grid[next_pos].used = target_amount;
        grid[target_pos].used = 0;
        step_count += 1;

        empty_cell = target_pos;
        clean_blacklist.remove(&target_pos);
        target_pos = next_pos;
    }
    pv!(step_count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let parsed = input
        .lines()
        .skip(2)
        .map(|l| {
            #[rustfmt::skip]
            let ret = scanf_unescaped!(l, r"/dev/grid/node-x{}-y{}\s+{}T\s+{}T\s+{}T\s+{}%", usize, usize, usize, usize, usize, usize).unwrap();
            ret
        })
        .inspect(|(x, y, size, used, avail, use_pct)| {
            assert_eq!(used + avail, *size);
            assert_eq!(used * 100 / size, *use_pct);
        })
        .map(|(_, _, size, used, _, _)| Node {
            size,
            used,
        })
        .to_vec();

    let target_node = {};

    let mut cnt = 0;
    for (i, a) in parsed.iter().enumerate() {
        for (j, b) in parsed.iter().enumerate() {
            if i == j {
                continue;
            }
            if a.used > 0 && b.used + a.used <= b.size {
                cnt += 1;
            }
        }
    }
    pv!(cnt);
}
