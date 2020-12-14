use regex::Regex;
use snafu::OptionExt;
use snafu::Snafu;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("input").lines();

    let parsed = input
        .map(|l| l.parse::<Instruction>())
        .filter_map(|p| p.ok())
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", part1(&parsed));
    println!("Part 2: {:?}", part2(&parsed));
}

fn part1(instructions: &Vec<Instruction>) -> u64 {
    let mut mem = HashMap::new();
    let mut current_mask = &vec![];
    for inst in instructions {
        match inst {
            Instruction::Mask { m } => current_mask = m,
            Instruction::Mem { loc, mut value } => {
                for (idx, bit) in current_mask.iter().rev().enumerate() {
                    value = match bit {
                        Some(true) => value | (1 << idx),
                        Some(false) => value & (!(1 << idx)),
                        None => value,
                    }
                }
                mem.insert(loc, value);
            }
        }
    }
    mem.values().sum()
}

fn part2(instructions: &Vec<Instruction>) -> u64 {
    let mut mem = HashMap::new();
    let mut current_mask = &vec![];

    for inst in instructions {
        match inst {
            Instruction::Mask { m } => current_mask = m,
            Instruction::Mem { loc, value } => {
                let mut poss_addresses = vec![*loc];
                for (idx, bit) in current_mask.iter().rev().enumerate() {
                    let bits = 1 << idx;
                    match bit {
                        Some(true) => {
                            for addr in &mut poss_addresses {
                                *addr |= bits;
                            }
                        }
                        Some(false) => {}
                        None => {
                            // this takes into account every possible variation of the existing variations
                            // out there, (its a lot of combinations!!)
                            for addr in poss_addresses.clone() {
                                poss_addresses.push(addr ^ bits);
                            }
                        }
                    }
                }
                // end of bit iteration, time to set the memory at all possible addresses
                for addr in poss_addresses {
                    mem.insert(addr, *value);
                }
            }
        }
    }

    mem.values().sum()
}

#[derive(Clone, Debug, Snafu)]
enum ParseInstructionErr {
    #[snafu(display("Could not parse data ({})", data))]
    Format { data: String },
}

#[derive(Debug)]
enum Instruction {
    Mem { loc: u64, value: u64 },
    Mask { m: Vec<Option<bool>> },
}

impl FromStr for Instruction {
    type Err = ParseInstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix("mask = ") {
            Some(mask) => parse_mask(mask),
            None => parse_mem(s),
        }
    }
}

fn parse_mem(mem: &str) -> Result<Instruction, ParseInstructionErr> {
    let re = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
    let captures = re.captures(mem).with_context(|| Format {
        data: String::from(mem),
    })?;
    Ok(Instruction::Mem {
        loc: captures.get(1).unwrap().as_str().parse().unwrap(),
        value: captures.get(2).unwrap().as_str().parse().unwrap(),
    })
}

fn parse_mask(mask: &str) -> Result<Instruction, ParseInstructionErr> {
    Ok(Instruction::Mask {
        m: mask
            .chars()
            .map(|c| match c {
                '0' => Some(false),
                '1' => Some(true),
                'X' => None,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>(),
    })
}
