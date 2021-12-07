use crate::utils::*;

#[derive(Debug, Clone, Copy)]
enum Parameter {
    Variable(char),
    Value(isize),
}
impl FromStr for Parameter {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            let c = s.chars().next().unwrap();
            if ('a'..='z').contains(&c) {
                return Ok(Parameter::Variable(c));
            }
        }
        s.parse::<isize>().map(Parameter::Value)
    }
}

enum Command {
    Copy,
    Inc,
    Dec,
    Jnz,
}
impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cpy" => Ok(Command::Copy),
            "inc" => Ok(Command::Inc),
            "dec" => Ok(Command::Dec),
            "jnz" => Ok(Command::Jnz),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}

struct Instruction {
    command: Command,
    params: Vec<Parameter>,
}
impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let command = parts.next().ok_or("No command")?.parse()?;
        let params = parts
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("{:?}", e))?;
        Ok(Instruction { command, params })
    }
}

fn get_value(registers: &HashMap<char, isize>, param: Parameter) -> isize {
    match param {
        Parameter::Variable(c) => registers[&c],
        Parameter::Value(v) => v,
    }
}
fn get_value_mut(registers: &mut HashMap<char, isize>, param: Parameter) -> &mut isize {
    match param {
        Parameter::Variable(c) => registers.get_mut(&c).unwrap(),
        Parameter::Value(_) => panic!("Value passed as parameter"),
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let parsed = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .to_vec();

    let mut registers = [('a', 0), ('b', 0), ('c', 1), ('d', 0isize)]
        .iter()
        .copied()
        .to_map();
    let mut ip = 0;

    while let Some(instr) = ip.try_into().ok().and_then(|n: usize| parsed.get(n)) {
        match instr.command {
            Command::Copy => {
                let src = get_value(&registers, instr.params[0]);
                *get_value_mut(&mut registers, instr.params[1]) = src;
            }
            Command::Inc => {
                *get_value_mut(&mut registers, instr.params[0]) += 1;
            }
            Command::Dec => {
                *get_value_mut(&mut registers, instr.params[0]) -= 1;
            }
            Command::Jnz => {
                let src = get_value(&registers, instr.params[0]);
                if src != 0 {
                    ip += get_value(&registers, instr.params[1]) - 1;
                }
            }
        }
        ip += 1;
    }
    pv!(registers[&'a']);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let parsed = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .to_vec();

    let mut registers = [('a', 0), ('b', 0), ('c', 0), ('d', 0isize)]
        .iter()
        .copied()
        .to_map();
    let mut ip = 0;

    while let Some(instr) = ip.try_into().ok().and_then(|n: usize| parsed.get(n)) {
        match instr.command {
            Command::Copy => {
                let src = get_value(&registers, instr.params[0]);
                *get_value_mut(&mut registers, instr.params[1]) = src;
            }
            Command::Inc => {
                *get_value_mut(&mut registers, instr.params[0]) += 1;
            }
            Command::Dec => {
                *get_value_mut(&mut registers, instr.params[0]) -= 1;
            }
            Command::Jnz => {
                let src = get_value(&registers, instr.params[0]);
                if src != 0 {
                    ip += get_value(&registers, instr.params[1]) - 1;
                }
            }
        }
        ip += 1;
    }
    pv!(registers[&'a']);
}
