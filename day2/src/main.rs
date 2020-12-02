use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PARSINGREGEX: Regex = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
}

fn main() {
    let input = include_str!("input");
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
struct Password {
    password: String,
    min: usize,
    max: usize,
    character: char,
}

impl Password {
    fn new(line: &str) -> Self {
        let captures = PARSINGREGEX.captures(line).unwrap();
        Password {
            min: captures.get(1).unwrap().as_str().parse().unwrap(),
            max: captures.get(2).unwrap().as_str().parse().unwrap(),
            character: captures.get(3).unwrap().as_str().chars().nth(0).unwrap(),
            password: captures.get(4).unwrap().as_str().to_string(),
        }
    }

    fn followed_corporate_policy(&self) -> bool {
        let count = self.password.matches(self.character).count();
        count >= self.min && count <= self.max
    }

    fn followed_revised_corporate_policy(&self) -> bool {
        let a = self.password.chars().nth(self.min - 1).unwrap();
        let b = self.password.chars().nth(self.max - 1).unwrap();

        (a == self.character && b != self.character) || (a != self.character && b == self.character)
    }
}
