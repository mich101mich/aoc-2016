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
        if ip as usize + 5 <= parsed.len() {
            let parsed = &parsed[ip as usize..][..5];
            let case_a = parsed[0].command == Inc
                && parsed[1].command == Dec
                && parsed[1].params[0] == parsed[2].params[0];
            let case_b = parsed[0].command == Dec
                && parsed[1].command == Inc
                && parsed[0].params[0] == parsed[2].params[0];

            if parsed[2].command == Jnz
                && (case_a || case_b)
                && parsed[2].params[1] == Parameter::Value(-2)
            {
                let (target, src) = if case_a { (0, 1) } else { (1, 0) };
                let a = get_value(&registers, parsed[src].params[0]);

                if parsed[4].command == Jnz
                    && parsed[3].command == Dec
                    && parsed[3].params[0] == parsed[4].params[0]
                    && parsed[4].params[1] == Parameter::Value(-5)
                {
                    // multiplication
                    let b = get_value(&registers, parsed[3].params[0]);
                    if let Some(target) = get_value_mut(&mut registers, parsed[target].params[0]) {
                        *target += a * b;
                        *get_value_mut(&mut registers, parsed[src].params[0]).unwrap() = 0;
                        *get_value_mut(&mut registers, parsed[3].params[0]).unwrap() = 0;
                        ip += 5;
                        continue;
                    }
                } else if let Some(target) = get_value_mut(&mut registers, parsed[target].params[0])
                {
                    // addition
                    *target += a;
                    *get_value_mut(&mut registers, parsed[src].params[0]).unwrap() = 0;
                    ip += 3;
                    continue;
                }
            }
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
