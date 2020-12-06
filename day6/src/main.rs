use std::collections::HashSet;

fn main() {
    let input = String::from(include_str!("input"));

    let p1 = input.split("\n\n").fold(0, |acc, g| {
        acc + g
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<HashSet<char>>()
            .len()
    });

    println!("part 1: {:#?}", p1);

    let p2 = input
        .split("\n\n")
        .map(|g| g.lines().collect::<Vec<&str>>())
        .map(|g| count_all_answered(&g))
        .sum::<usize>();

    println!("part 2: {:#?}", p2);
}

fn count_all_answered(group: &Vec<&str>) -> usize {
    let alphabet = (b'a'..=b'z').map(char::from);
    alphabet.fold(0, |acc, c| {
        acc + all_people_answered_yes_to(group, &c) as usize
    })
}

fn all_people_answered_yes_to(group: &Vec<&str>, c: &char) -> bool {
    group
        .iter()
        .map(|g| g.chars().collect::<Vec<char>>())
        .all(|g| g.contains(&c))
}
