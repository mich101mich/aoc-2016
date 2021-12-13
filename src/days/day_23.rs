use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Copy,
    Inc,
    Dec,
    Jnz,
    Tgl,
}
use Command::*;
impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cpy" => Ok(Copy),
            "inc" => Ok(Inc),
            "dec" => Ok(Dec),
            "jnz" => Ok(Jnz),
            "tgl" => Ok(Tgl),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
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

fn get_instr(instructions: &[Instruction], ip: isize) -> Option<&Instruction> {
    if ip >= 0 {
        instructions.get(ip as usize)
    } else {
        None
    }
}
fn get_value(registers: &HashMap<char, isize>, param: Parameter) -> isize {
    match param {
        Parameter::Variable(c) => registers[&c],
        Parameter::Value(v) => v,
    }
}
fn get_value_mut(registers: &mut HashMap<char, isize>, param: Parameter) -> Option<&mut isize> {
    match param {
        Parameter::Variable(c) => Some(registers.get_mut(&c).unwrap()),
        Parameter::Value(_) => None,
    }
}

fn detect_advanced_ops(
    instr: &[Instruction],
    ip: usize,
    reg: &mut HashMap<char, isize>,
) -> Option<usize> {
    if ip + 5 > instr.len() {
        return None;
    }
    let instr = &instr[ip..][..5];
    let case_a = instr[0].command == Inc
        && instr[1].command == Dec
        && instr[1].params[0] == instr[2].params[0];
    let case_b = instr[0].command == Dec
        && instr[1].command == Inc
        && instr[0].params[0] == instr[2].params[0];

    if instr[2].command != Jnz || (!case_a && !case_b) || instr[2].params[1] != Parameter::Value(-2)
    {
        return None;
    }
    let (target, src) = if case_a { (0, 1) } else { (1, 0) };
    let a = get_value(reg, instr[src].params[0]);
    if instr[4].command == Jnz
        && instr[3].command == Dec
        && instr[3].params[0] == instr[4].params[0]
        && instr[4].params[1] == Parameter::Value(-5)
    {
        // multiplication
        let b = get_value(reg, instr[3].params[0]);
        if let Some(target) = get_value_mut(reg, instr[target].params[0]) {
            *target += a * b;
            *get_value_mut(reg, instr[src].params[0]).unwrap() = 0;
            *get_value_mut(reg, instr[3].params[0]).unwrap() = 0;
            return Some(5);
        }
    } else {
        // addition
        if let Some(target) = get_value_mut(reg, instr[target].params[0]) {
            *target += a;
            *get_value_mut(reg, instr[src].params[0]).unwrap() = 0;
            return Some(3);
        }
    }
    None
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let mut parsed = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .to_vec();

    let mut registers = ('a'..='d').map(|c| (c, 0isize)).to_map();
    let mut ip = 0;

    *registers.get_mut(&'a').unwrap() = 12;

    while let Some(instr) = get_instr(&parsed, ip) {
        if let Some(jump) = detect_advanced_ops(&parsed, ip as usize, &mut registers) {
            ip += jump as isize;
            continue;
        }
        match instr.command {
            Copy => {
                let src = get_value(&registers, instr.params[0]);
                if let Some(v) = get_value_mut(&mut registers, instr.params[1]) {
                    *v = src;
                }
            }
            Inc => {
                if let Some(v) = get_value_mut(&mut registers, instr.params[0]) {
                    *v += 1;
                }
            }
            Dec => {
                if let Some(v) = get_value_mut(&mut registers, instr.params[0]) {
                    *v -= 1;
                }
            }
            Jnz => {
                let src = get_value(&registers, instr.params[0]);
                if src != 0 {
                    ip += get_value(&registers, instr.params[1]) - 1;
                }
            }
            Tgl => {
                let src = ip + get_value(&registers, instr.params[0]);
                if let Some(instr) = get_instr(&parsed, src).cloned() {
                    let src = src as usize;
                    if instr.params.len() == 1 {
                        let cmd = match instr.command {
                            Inc => Dec,
                            _ => Inc,
                        };
                        parsed[src].command = cmd;
                    } else {
                        let cmd = match instr.command {
                            Jnz => Copy,
                            _ => Jnz,
                        };
                        parsed[src].command = cmd;
                    }
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
    let input = include_str!("../input/23.txt");

    let mut parsed = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .to_vec();

    let mut registers = ('a'..='d').map(|c| (c, 0isize)).to_map();
    let mut ip = 0;

    *registers.get_mut(&'a').unwrap() = 7;

    while let Some(instr) = get_instr(&parsed, ip) {
        match instr.command {
            Copy => {
                let src = get_value(&registers, instr.params[0]);
                if let Some(v) = get_value_mut(&mut registers, instr.params[1]) {
                    *v = src;
                }
            }
            Inc => {
                if let Some(v) = get_value_mut(&mut registers, instr.params[0]) {
                    *v += 1;
                }
            }
            Dec => {
                if let Some(v) = get_value_mut(&mut registers, instr.params[0]) {
                    *v -= 1;
                }
            }
            Jnz => {
                let src = get_value(&registers, instr.params[0]);
                if src != 0 {
                    ip += get_value(&registers, instr.params[1]) - 1;
                }
            }
            Tgl => {
                let src = ip + get_value(&registers, instr.params[0]);
                if let Some(instr) = get_instr(&parsed, src).cloned() {
                    let src = src as usize;
                    if instr.params.len() == 1 {
                        let cmd = match instr.command {
                            Inc => Dec,
                            _ => Inc,
                        };
                        parsed[src].command = cmd;
                    } else {
                        let cmd = match instr.command {
                            Jnz => Copy,
                            _ => Jnz,
                        };
                        parsed[src].command = cmd;
                    }
                }
            }
        }
        ip += 1;
    }
    pv!(registers[&'a']);
}
