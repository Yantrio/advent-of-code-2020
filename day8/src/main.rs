use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}", pt1(input));
    println!("Part 2: {}", pt2(input));
}

fn pt1(input: &str) -> i64 {
    let mut c = input.parse::<Computer>().unwrap();
    c.run_to_end_or_loop().err().unwrap()
}

fn pt2(input: &str) -> i64 {
    let reference = input.parse::<Computer>().unwrap().instructions;
    // make a reference program
    reference
        .iter()
        .enumerate()
        .filter(|(_, instr)| instr.opcode == OpCode::Jmp || instr.opcode == OpCode::Nop)
        .map(|(idx, _)| (input.parse::<Computer>().unwrap(), idx))
        .map(|(mut c, idx)| {
            // flip nop and jmp at the idx
            flip_at_idx(&mut c, idx);
            c.run_to_end_or_loop()
        })
        .find(|r| r.is_ok())
        .map(|r| r.unwrap())
        .unwrap()
}

fn flip_at_idx(c: &mut Computer, idx: usize) {
    let i = c.instruction_at(idx);
    i.opcode = match i.opcode {
        OpCode::Nop => OpCode::Jmp,
        OpCode::Jmp => OpCode::Nop,
        _ => unreachable!(), // should never be called with anything else due to the filter
    }
}

#[derive(Debug, PartialEq)]
#[repr(i32)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for OpCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(OpCode::Acc),
            "jmp" => Ok(OpCode::Jmp),
            "nop" => Ok(OpCode::Nop),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: OpCode,
    argument: i64,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = s.split_ascii_whitespace().collect::<Vec<&str>>();
        Ok(Instruction {
            opcode: op[0].parse().unwrap(),
            argument: op[1].parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Computer {
    pub ip: i64,
    pub acc: i64,
    pub instructions: Vec<Instruction>,
}

impl FromStr for Computer {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Computer {
            ip: 0,
            acc: 0,
            instructions: s.lines().map(|s| s.parse().unwrap()).collect(),
        })
    }
}

impl Computer {
    fn instruction_at(&mut self, idx: usize) -> &mut Instruction {
        &mut self.instructions[idx]
    }

    fn current_instruction(&mut self) -> &Instruction {
        self.instruction_at(self.ip as usize)
    }

    fn step(&mut self) {
        let curr = self.current_instruction();
        match curr.opcode {
            OpCode::Acc => {
                self.acc += curr.argument;
                self.ip += 1;
            }
            OpCode::Jmp => self.ip += curr.argument,
            OpCode::Nop => self.ip += 1,
        }
    }

    fn run_to_end_or_loop(&mut self) -> Result<i64, i64> {
        let mut seen_ips: HashSet<i64> = HashSet::new();
        loop {
            match (seen_ips.contains(&self.ip), (self.is_done())) {
                (true, _) => return Err(self.acc),
                (_, true) => return Ok(self.acc),
                _ => {
                    seen_ips.insert(self.ip);
                    self.step();
                }
            }
        }
    }

    fn is_done(&mut self) -> bool {
        self.ip >= self.instructions.len() as i64
    }
}

#[cfg(test)]
mod day8 {
    use super::Instruction;
    use super::OpCode;

    fn test_parse(str: &str, expected: Instruction) {
        let res = str.parse::<Instruction>().unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn should_parse_instructions() {
        test_parse(
            "nop +0",
            Instruction {
                opcode: OpCode::Nop,
                argument: 0,
            },
        );
        test_parse(
            "acc +1",
            Instruction {
                opcode: OpCode::Acc,
                argument: 1,
            },
        );
        test_parse(
            "jmp -1",
            Instruction {
                opcode: OpCode::Jmp,
                argument: -1,
            },
        );
    }
}
