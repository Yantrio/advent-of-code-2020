fn main() {
    let input = include_str!("input");
    println!("Part 1: {}", pt1(input));
    println!("Part 2: {}", pt2(input).unwrap());
}

fn pt1(input: &str) -> i64 {
    let c = &mut Computer::new(input);

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
            _ => unreachable!(),
        }
    };
    Computer::new(input) // make a reference program
        .instructions
        .iter()
        .enumerate()
        .filter(|(_, instr)| instr.opcode == OpCode::Jmp || instr.opcode == OpCode::Nop) //find all the ops we need to check
        .map(|(idx, _)| (Computer::new(input), idx)) // create a new program for them
        .map(|(mut c, idx)| {
            // test the jmp<==>nop conversion
            flip_op(&mut c.instructions[idx as usize]);
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

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: OpCode,
    argument: i64,
}

#[derive(Debug)]
struct Computer {
    pub ip: i64,
    pub acc: i64,
    pub instructions: Vec<Instruction>,
}

impl Computer {
    fn parse_op_code(input: &str) -> OpCode {
        match input {
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            "nop" => OpCode::Nop,
            _ => unreachable!(),
        }
    }

    fn new(input: &str) -> Self {
        Computer {
            ip: 0,
            acc: 0,
            instructions: input
                .lines()
                .map(|l| Computer::parse_operation(l))
                .collect(),
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
    }

    fn print(&self) {
        println!("IP {}, ACC {}", self.ip, self.acc);
    }

    fn parse_operation(input: &str) -> Instruction {
        let op = input.split_ascii_whitespace().collect::<Vec<&str>>();
        Instruction {
            opcode: Computer::parse_op_code(op[0]),
            argument: op[1].parse().unwrap(),
        }
    }

    fn step(&mut self) {
        let curr = &self.instructions[self.ip as usize];
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
                true => return false, // we dont have comparisons yet
                false => match self.ip >= self.instructions.len() as i64 {
                    true => return true,
                    false => {
                        seen_ips.push(self.ip);
                        self.step();
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod day8 {
    use super::Computer;
    use super::Instruction;
    use super::OpCode;

    fn test_parse(str: &str, expected: Instruction) {
        let res = Computer::parse_operation(str);
        assert_eq!(res, expected);
    }

    #[test]
    fn should_parse_opcodes() {
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
