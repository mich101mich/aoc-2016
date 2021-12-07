use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let parsed = parse_u(input);

    let mut neighborhood = ManhattanNeighborhood::new(100_000, 100_000);
    let mut pos = (1, 1);

    fn is_open((x, y): Point, fav_num: usize) -> bool {
        let v = x * x + 3 * x + 2 * x * y + y + y * y + fav_num;
        v.count_ones() & 1 == 0
    }

    let goals = (0..=51)
        .flat_map(|y| (0..=51).map(move |x| (x, y)))
        .filter(|&(x, y)| is_open((x, y), parsed))
        .filter(|p| manhattan(*p, pos) <= 50)
        .to_vec();

    let mut paths = dijkstra_search(
        |pos, out| {
            for p in neighborhood.get_all_neighbors(pos) {
                if is_open(p, parsed) {
                    out.push(p);
                }
            }
        },
        pos,
        &goals,
    );
    paths.retain(|_, p| p.cost <= 50);
    pv!(paths.len());
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let parsed = parse_u(input);

    let mut neighborhood = ManhattanNeighborhood::new(100_000, 100_000);
    let mut pos = (1, 1);

    fn is_open((x, y): Point, fav_num: usize) -> bool {
        let v = x * x + 3 * x + 2 * x * y + y + y * y + fav_num;
        v.count_ones() & 1 == 0
    }

    let goal = (31, 39);
    let path = a_star_search(
        |pos, out| {
            for p in neighborhood.get_all_neighbors(pos) {
                if is_open(p, parsed) {
                    out.push(p);
                }
            }
        },
        pos,
        goal,
        |p| neighborhood.heuristic(p, goal),
    );
    let path = path.unwrap();
    pv!(path.cost);
}
