use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    fn append(s: &[char], set: &mut HashSet<(char, char)>, reverse: bool) {
        s.windows(3)
            .filter(|w| w[0] == w[2] && w[0] != w[1])
            .for_each(|w| {
                set.insert(if reverse { (w[1], w[0]) } else { (w[0], w[1]) });
            });
    }

    let parsed = input
        .lines()
        .filter(|l| {
            let mut inside = HashSet::new();
            let mut outside = HashSet::new();

            let mut s = vec![];
            for c in l.chars() {
                if c == '[' {
                    append(&s, &mut inside, false);
                    s.clear();
                } else if c == ']' {
                    append(&s, &mut outside, true);
                    s.clear();
                } else {
                    s.push(c);
                }
            }
            append(&s, &mut inside, false);

            inside.intersection(&outside).next().is_some()
        })
        .count();
    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    enum State {
        Nothing,
        One(char),
        Two(char, char),
        Three(char, char),
        Four,
    }
    impl State {
        fn advance(&mut self, c: char) {
            *self = match *self {
                State::Nothing => State::One(c),

                State::One(c1) if c1 != c => State::Two(c1, c),
                State::One(_) => State::One(c),

                State::Two(c1, c2) if c2 == c => State::Three(c1, c2),
                State::Two(_, c2) => State::Two(c2, c),

                State::Three(c1, c2) if c1 == c => State::Four,
                State::Three(_, c2) if c2 != c => State::Two(c2, c),
                State::Three(_, _) => State::One(c),

                State::Four => State::Four,
            }
        }
        fn has_found(&self) -> bool {
            matches!(*self, State::Four)
        }
    }

    let parsed = input
        .lines()
        .filter(|l| {
            let mut found = false;
            let mut state = State::Nothing;
            for c in l.chars() {
                if c == '[' {
                    found = found || state.has_found();
                    state = State::Nothing;
                } else if c == ']' {
                    if state.has_found() {
                        return false;
                    }
                    state = State::Nothing;
                } else {
                    state.advance(c);
                }
            }
            found = found || state.has_found();
            found
        })
        .count();
    pv!(parsed);

    // Naive Solution:

    // fn is_abba(w: &[char]) -> bool {
    //     w[0] == w[3] && w[1] == w[2] && w[0] != w[1]
    // }
    // fn check(v: &[char]) -> bool {
    //     v.windows(4).any(is_abba)
    // }

    // let parsed = input
    //     .lines()
    //     .filter(|l| {
    //         let mut found = false;
    //         let mut s = vec![];
    //         for c in l.chars() {
    //             if c == '[' {
    //                 found = found || check(&s);
    //                 s.clear();
    //             } else if c == ']' {
    //                 if check(&s) {
    //                     return false;
    //                 }
    //                 s.clear();
    //             } else {
    //                 s.push(c);
    //             }
    //         }
    //         found = found || check(&s);
    //         found
    //     })
    //     .count();
    // pv!(parsed);
}
