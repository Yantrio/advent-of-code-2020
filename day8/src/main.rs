use std::str::FromStr;

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}", pt1(input));
    println!("Part 2: {}", pt2(input).unwrap());
}

fn pt1(input: &str) -> i64 {
    let c: &mut Computer = &mut input.parse().unwrap();

    let mut seen_ips: Vec<i64> = Vec::new();
    while !seen_ips.contains(&c.ip) {
        seen_ips.push(c.ip);
        c.step();
    }
    return c.acc;
}

fn pt2(input: &str) -> Option<i64> {
    let flip_op = |i: &mut Instruction| {
        i.opcode = match i.opcode {
            OpCode::Nop => OpCode::Jmp,
            OpCode::Jmp => OpCode::Nop,
            _ => unreachable!(), // should never be called with anything else due to the filter
        }
    };

    input
        .parse::<Computer>()
        .unwrap() // make a reference program
        .instructions
        .iter()
        .enumerate()
        .filter(|(_, instr)| instr.opcode == OpCode::Jmp || instr.opcode == OpCode::Nop)
        .map(|(idx, _)| (input.parse::<Computer>().unwrap(), idx))
        .map(|(mut c, idx)| {
            // flip nop and jmp at the idx
            flip_op(c.instruction_at(idx));
            c
        })
        .map(|mut c| {
            // test the swapped operator
            match c.does_terminate() {
                true => Some(c.acc), // return the accumulator if it completes
                false => None,
            }
        })
        .find(|c| c.is_some())
        .unwrap()
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
    fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
    }

    fn print(&self) {
        println!("IP {}, ACC {}", self.ip, self.acc);
    }

    fn instruction_at(&mut self, idx: usize) -> &mut Instruction {
        &mut self.instructions[idx as usize]
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

    fn does_terminate(&mut self) -> bool {
        let mut seen_ips: Vec<i64> = Vec::new();
        loop {
            match seen_ips.contains(&self.ip) {
                true => return false, // looped, because we dont have comparisons, we know this wont return!
                false => match self.is_done() {
                    true => return true,
                    false => {
                        seen_ips.push(self.ip);
                        self.step();
                    }
                },
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
