use std::collections::HashSet;

fn main() {
    let groups = || include_str!("input").split("\n\n");

    let p1 = groups().fold(0, |acc, g| acc + count_unique_chars(g));
    println!("part 1: {:#?}", p1);

    let p2: usize = groups()
        .map(|g| g.lines().collect::<Vec<&str>>())
        .map(count_all_answered)
        .sum();
    println!("part 2: {:#?}", p2);
}

fn count_unique_chars(input: &str) -> usize {
    input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<HashSet<_>>()
        .len()
}

fn count_all_answered(group: Vec<&str>) -> usize {
    let alphabet = (b'a'..=b'z').map(char::from);
    alphabet.fold(0, |acc, c| acc + all_answered(&group, &c) as usize)
}

fn all_answered(group: &Vec<&str>, c: &char) -> bool {
    group
        .iter()
        .map(|&g| g.chars().collect::<Vec<char>>())
        .all(|g| g.contains(&c))
}
