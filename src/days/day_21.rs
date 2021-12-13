use crate::utils::*;

#[derive(Debug, Clone, Copy)]
enum Action {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}
use Action::*;

impl Action {
    fn apply(&self, s: &mut [char]) {
        match *self {
            SwapPosition(a, b) => s.swap(a, b),
            SwapLetter(a, b) => {
                s.iter_mut().for_each(|c| {
                    if *c == a {
                        *c = b;
                    } else if *c == b {
                        *c = a;
                    }
                });
            }
            RotateLeft(a) => s.rotate_left(a),
            RotateRight(a) => s.rotate_right(a),
            RotateLetter(a) => {
                let mut shift = s.iter().position(|x| *x == a).unwrap() + 1;
                if shift >= 5 {
                    shift = (shift + 1) % s.len();
                }
                s.rotate_right(shift);
            }
            Reverse(a, b) => s[a..=b].reverse(),
            Move(a, b) => {
                if a < b {
                    s[a..=b].rotate_left(1)
                } else {
                    s[b..=a].rotate_right(1)
                }
            }
        }
    }
    fn un_apply(&self, s: &mut [char]) {
        match *self {
            SwapPosition(a, b) => s.swap(a, b),
            SwapLetter(a, b) => {
                s.iter_mut().for_each(|c| {
                    if *c == a {
                        *c = b;
                    } else if *c == b {
                        *c = a;
                    }
                });
            }
            RotateLeft(a) => s.rotate_right(a),
            RotateRight(a) => s.rotate_left(a),
            RotateLetter(a) => {
                let final_pos = s.iter().position(|x| *x == a).unwrap();
                let (_, shift) = (0..s.len())
                    .map(|pos| {
                        let mut shift = pos + 1;
                        if shift >= 5 {
                            shift = (shift + 1) % s.len();
                        }
                        (pos, shift)
                    })
                    .find(|(pos, cnt)| (pos + cnt) % s.len() == final_pos)
                    .unwrap();
                s.rotate_left(shift);
            }
            Reverse(a, b) => s[a..=b].reverse(),
            Move(a, b) => {
                if a < b {
                    s[a..=b].rotate_right(1)
                } else {
                    s[b..=a].rotate_left(1)
                }
            }
        }
    }
}
impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ret = if let Some((a, b)) = scanf!(s, "swap position {} with position {}", usize, usize)
        {
            SwapPosition(a, b)
        } else if let Some((a, b)) = scanf!(s, "swap letter {} with letter {}", char, char) {
            SwapLetter(a, b)
        } else if let Some((a, _)) = scanf!(s, "rotate left {} ste{}", usize, String) {
            RotateLeft(a)
        } else if let Some((a, _)) = scanf!(s, "rotate right {} ste{}", usize, String) {
            RotateRight(a)
        } else if let Some(a) = scanf!(s, "rotate based on position of letter {}", char) {
            RotateLetter(a)
        } else if let Some((a, b)) = scanf!(s, "reverse positions {} through {}", usize, usize) {
            Reverse(a, b)
        } else if let Some((a, b)) = scanf!(s, "move position {} to position {}", usize, usize) {
            Move(a, b)
        } else {
            return Err(format!("invalid action: {}", s));
        };
        Ok(ret)
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let parsed = input.lines().map(|l| l.parse::<Action>().unwrap()).to_vec();

    let mut pw = "fbgdceah".chars().to_vec();
    for a in parsed.iter().rev() {
        a.un_apply(&mut pw);
    }
    print_arr!(pw);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let parsed = input.lines().map(|l| l.parse::<Action>().unwrap()).to_vec();

    let mut pw = "abcdefgh".chars().to_vec();
    for a in &parsed {
        a.apply(&mut pw);
    }
    print_arr!(pw);
}
