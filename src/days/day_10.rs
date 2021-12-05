use crate::utils::*;

enum Event {
    ValueGoesTo(usize, usize),
    BotGives {
        bot: usize,
        low: (bool, usize),
        high: (bool, usize),
    },
}
impl RegexRepresentation for Event {
    const REGEX: &'static str = r"value \d+ goes to bot \d+|bot \d+ gives low to (bot|output) \d+ and high to (bot|output) \d+";
}
impl std::str::FromStr for Event {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = scanf!(s, "value {} goes to bot {}", usize, usize) {
            Ok(Event::ValueGoesTo(a, b))
        } else if let Some((bot, o1, o1v, o2, o2v)) = scanf!(
            s,
            "bot {} gives low to {} {} and high to {} {}",
            usize,
            String,
            usize,
            String,
            usize
        ) {
            Ok(Event::BotGives {
                bot,
                low: (o1 == "bot", o1v),
                high: (o2 == "bot", o2v),
            })
        } else {
            Err(())
        }
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut parsed = input
        .lines()
        .map(|l| scanf!(l, "{}", Event).unwrap())
        .to_vec();

    let mut bots = vec![];
    let mut outputs = vec![];

    fn insert(out: &mut Vec<Vec<usize>>, bot: usize, val: usize) {
        if out.len() <= bot {
            out.resize(bot + 1, vec![]);
        }
        let o = &mut out[bot];
        o.push(val);
        o.sort_unstable();
    }

    while !parsed.is_empty() {
        parsed.retain(|i| match i {
            Event::ValueGoesTo(v, b) => {
                if bots.len() <= *b {
                    bots.resize(b + 1, vec![]);
                }
                insert(&mut bots, *b, *v);
                false
            }
            Event::BotGives { bot, low, high } => {
                if bots.get(*bot).map(|b| b.len()).unwrap_or(0) != 2 {
                    // not done yet
                    return true;
                }
                let mut bot = bots[*bot].clone();
                let low_target = if low.0 { &mut bots } else { &mut outputs };
                insert(low_target, low.1, bot[0]);
                let high_target = if high.0 { &mut bots } else { &mut outputs };
                insert(high_target, high.1, bot[1]);
                false
            }
        });
    }
    let res = outputs[0][0] * outputs[1][0] * outputs[2][0];
    pv!(res);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut parsed = input
        .lines()
        .map(|l| scanf!(l, "{}", Event).unwrap())
        .to_vec();

    let mut bots = vec![];
    let mut outputs = vec![];

    fn insert(out: &mut Vec<Vec<usize>>, bot: usize, val: usize) {
        if out.len() <= bot {
            out.resize(bot + 1, vec![]);
        }
        let o = &mut out[bot];
        o.push(val);
        o.sort_unstable();
    }

    while !parsed.is_empty() {
        parsed.retain(|i| match i {
            Event::ValueGoesTo(v, b) => {
                if bots.len() <= *b {
                    bots.resize(b + 1, vec![]);
                }
                insert(&mut bots, *b, *v);
                false
            }
            Event::BotGives { bot, low, high } => {
                if bots.get(*bot).map(|b| b.len()).unwrap_or(0) != 2 {
                    // not done yet
                    return true;
                }
                let mut bot = bots[*bot].clone();
                let low_target = if low.0 { &mut bots } else { &mut outputs };
                insert(low_target, low.1, bot[0]);
                let high_target = if high.0 { &mut bots } else { &mut outputs };
                insert(high_target, high.1, bot[1]);
                false
            }
        });
    }
    let target = bots
        .iter()
        .enumerate()
        .find(|(_, b)| b == &&[17, 61])
        .unwrap();
    pv!(target);
}
