use crate::utils::*;

fn calc_doors(key: &str, pos: (usize, usize), path: &str) -> [bool; 4] {
    let mut doors = [false; 4];
    let hash = format!("{:x}", md5::compute(format!("{}{}", key, path)));
    for (dir, c) in [Up, Down, Left, Right].iter().zip(hash.chars()) {
        let pos_valid = dir
            .checked_add(pos)
            .map(|(x, y)| x < 4 && y < 4)
            .unwrap_or(false);
        doors[dir.num()] = pos_valid && c >= 'b';
    }
    doors
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Element {
    pos: Point,
    path: String,
}
impl PartialOrd for Element {
    fn partial_cmp(&self, rhs: &Element) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for Element {
    fn cmp(&self, rhs: &Element) -> Ordering {
        self.path.len().cmp(&rhs.path.len()).reverse()
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let mut next = std::collections::BinaryHeap::new();
    next.push(Element {
        pos: (0, 0),
        path: String::new(),
    });

    let mut longest = 0;
    while let Some(Element { pos, path }) = next.pop() {
        if pos == (3, 3) {
            if path.len() > longest {
                longest = path.len();
            }
            continue;
        }
        let doors = calc_doors(input, pos, &path);
        for d in Dir::all() {
            if !doors[d.num()] {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(d.to_char());
            next.push(Element {
                pos: pos + d,
                path: new_path,
            });
        }
    }
    pv!(longest);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let mut next = std::collections::BinaryHeap::new();
    next.push(Element {
        pos: (0, 0),
        path: String::new(),
    });

    let mut final_path = String::new();
    while let Some(Element { pos, path }) = next.pop() {
        if pos == (3, 3) {
            final_path = path;
            break;
        }
        let doors = calc_doors(input, pos, &path);
        for d in Dir::all() {
            if !doors[d.num()] {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(d.to_char());
            next.push(Element {
                pos: pos + d,
                path: new_path,
            });
        }
    }
    pv!(final_path);
}
