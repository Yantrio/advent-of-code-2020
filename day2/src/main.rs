use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

lazy_static! {
    static ref PARSINGREGEX: Regex = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
}

fn main() {
    let input = read_to_string("input").expect("failed to read input file");
    let passwords = input.lines().map(Password::new).collect::<Vec<Password>>();

    let valid_passwords = passwords
        .iter()
        .filter(|p| p.followed_corporate_policy())
        .count();

    println!("Part 1: {:#?}", valid_passwords);

    let new_valid_passwords = passwords
        .iter()
        .filter(|p| p.followed_revised_corporate_policy())
        .count();

    println!("Part 2: {:#?}", new_valid_passwords);
}

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    character: char,
}

#[derive(Debug)]
struct Password {
    policy: Policy,
    password: String,
}

impl Password {
    fn new(line: &str) -> Self {
        let captures = PARSINGREGEX.captures(line).unwrap();
        Password {
            policy: Policy {
                min: captures.get(1).unwrap().as_str().parse().unwrap(),
                max: captures.get(2).unwrap().as_str().parse().unwrap(),
                character: captures.get(3).unwrap().as_str().chars().next().unwrap(),
            },
            password: captures.get(4).unwrap().as_str().to_string(),
        }
    }

    fn followed_corporate_policy(&self) -> bool {
        let count = self.password.matches(self.policy.character).count();
        count >= self.policy.min && count <= self.policy.max
    }

    fn followed_revised_corporate_policy(&self) -> bool {
        let a = self.password.chars().nth(self.policy.min - 1).unwrap();
        let b = self.password.chars().nth(self.policy.max - 1).unwrap();

        (a == self.policy.character && b != self.policy.character)
            || (a != self.policy.character && b == self.policy.character)
    }
}
