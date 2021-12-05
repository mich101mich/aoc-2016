use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Floor {
    chips: u32,
    generators: u32,
}
impl Floor {
    fn remove(&mut self, id: u32, is_chip: bool) {
        if is_chip {
            self.chips &= !(1 << id);
        } else {
            self.generators &= !(1 << id);
        }
    }
    fn add(&mut self, id: u32, is_chip: bool) {
        if is_chip {
            self.chips |= 1 << id;
        } else {
            self.generators |= 1 << id;
        }
    }
    fn is_legal(&self) -> bool {
        self.generators == 0 || (self.chips & !self.generators) == 0
    }
}

#[allow(unused)]
fn check_legal((_, floors): &(usize, [Floor; 4])) -> bool {
    floors.iter().all(|f| f.is_legal())
}
#[allow(unused)]
fn display((elevator, floors): &(usize, [Floor; 4]), num_elements: usize) {
    for (i, floor) in floors.iter().enumerate().rev() {
        let mut chips = String::new();
        let mut generators = String::new();
        for i in (0..num_elements).rev() {
            let c = floor.chips & (1 << i) != 0;
            chips.push(if c { 'x' } else { ' ' });
            let g = floor.generators & (1 << i) != 0;
            generators.push(if g { 'x' } else { ' ' });
        }
        let e = if i == *elevator { 'E' } else { ' ' };
        println!("F{} {} {}M {}G", i + 1, e, chips, generators);
    }
    println!();
}

fn find_path(floors: [Floor; 4], num_elements: usize) -> Path<(usize, [Floor; 4])> {
    let all_elements = (1 << num_elements) - 1;

    let mut goal = (3, [Floor::default(); 4]);
    goal.1[3].chips = all_elements;
    goal.1[3].generators = all_elements;

    let start = (0, floors);

    let path = a_star_search(
        |(elevator, floors), out| {
            let current_elements = floors[elevator];
            let elements = (0..num_elements)
                .filter(|i| (current_elements.chips >> i) & 1 == 1)
                .map(|i| (true, i))
                .chain(
                    (0..num_elements)
                        .filter(|i| (current_elements.generators >> i) & 1 == 1)
                        .map(|i| (false, i)),
                )
                .to_vec();
            for (i, e) in elements.iter().enumerate() {
                let (is_chip, id) = e;
                let id = *id as u32;
                let mut accessible = vec![];
                if elevator > 0 {
                    accessible.push(elevator - 1);
                }
                if elevator < 3 {
                    accessible.push(elevator + 1);
                }
                {
                    // move only e
                    let mut new_floors = floors;
                    new_floors[elevator].remove(id, *is_chip);
                    if new_floors[elevator].is_legal() {
                        for &e in accessible.iter() {
                            let mut new_floors = new_floors;
                            new_floors[e].add(id, *is_chip);
                            if new_floors[e].is_legal() {
                                out.push((e, new_floors));
                            }
                        }
                    }
                }
                for other in elements.iter().skip(i + 1) {
                    let (other_is_chip, other_id) = other;
                    let other_id = *other_id as u32;

                    let mut new_floors = floors;
                    new_floors[elevator].remove(id, *is_chip);
                    new_floors[elevator].remove(other_id, *other_is_chip);
                    if !new_floors[elevator].is_legal() {
                        continue;
                    }
                    for &e in accessible.iter() {
                        let mut new_floors = new_floors;
                        new_floors[e].add(id, *is_chip);
                        new_floors[e].add(other_id, *other_is_chip);
                        if new_floors[e].is_legal() {
                            out.push((e, new_floors));
                        }
                    }
                }
            }
        },
        start,
        goal,
        |(_, state)| {
            let done = state[3].chips.count_ones() + state[3].generators.count_ones();
            let missing = num_elements * 2 - done as usize;
            (missing as f32 / 2.0).ceil() as usize
        },
    );
    path.unwrap()
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let parsed = input.lines().map(|l| l.split(" contains ").nth(1).unwrap());

    let mut name_map = HashMap::new();
    let mut next_id = 0u32;
    let mut floors = [Floor::default(); 4];

    for (floor, items) in floors.iter_mut().zip(parsed) {
        for item in items
            .split(", and ")
            .flat_map(|s| s.split(" and "))
            .flat_map(|s| s.split(", "))
        {
            let item = item.strip_suffix('.').unwrap_or(item);
            let item = item.strip_prefix("a ").unwrap_or(item);
            if item == "nothing relevant" {
                continue;
            }
            let (is_chip, name) = if let Some(name) = item.strip_suffix(" generator") {
                (false, name)
            } else if let Some(name) = item.strip_suffix("-compatible microchip") {
                (true, name)
            } else {
                panic!("Unknown item: {}", item);
            };
            let id = match name_map.entry(name.to_string()) {
                Entry::Occupied(o) => *o.get(),
                Entry::Vacant(v) => {
                    let id = next_id;
                    next_id += 1;
                    v.insert(id);
                    id
                }
            };
            floor.add(id, is_chip);
        }
    }

    name_map.insert("elerium".to_string(), next_id);
    floors[0].add(next_id, true);
    floors[0].add(next_id, false);
    next_id += 1;
    name_map.insert("dilithium".to_string(), next_id);
    floors[0].add(next_id, true);
    floors[0].add(next_id, false);

    next_id += 1;

    let num_elements = name_map.len();
    let path = find_path(floors, num_elements);
    // for step in path.path {
    //     display(&step, num_elements);
    // }

    pv!(path.cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let parsed = input.lines().map(|l| l.split(" contains ").nth(1).unwrap());

    let mut name_map = HashMap::new();
    let mut next_id = 0u32;
    let mut floors = [Floor::default(); 4];

    for (floor, items) in floors.iter_mut().zip(parsed) {
        for item in items
            .split(", and ")
            .flat_map(|s| s.split(" and "))
            .flat_map(|s| s.split(", "))
        {
            let item = item.strip_suffix('.').unwrap_or(item);
            let item = item.strip_prefix("a ").unwrap_or(item);
            if item == "nothing relevant" {
                continue;
            }
            let (is_chip, name) = if let Some(name) = item.strip_suffix(" generator") {
                (false, name)
            } else if let Some(name) = item.strip_suffix("-compatible microchip") {
                (true, name)
            } else {
                panic!("Unknown item: {}", item);
            };
            let id = match name_map.entry(name.to_string()) {
                Entry::Occupied(o) => *o.get(),
                Entry::Vacant(v) => {
                    let id = next_id;
                    next_id += 1;
                    v.insert(id);
                    id
                }
            };
            floor.add(id, is_chip);
        }
    }

    let num_elements = name_map.len();
    let path = find_path(floors, num_elements);
    // for step in path.path {
    //     display(&step, num_elements);
    // }

    pv!(path.cost);
}
