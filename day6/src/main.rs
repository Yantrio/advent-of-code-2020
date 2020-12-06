use std::collections::HashSet;

fn main() {
    let input = String::from(include_str!("input"));

    let p1 = input
        .split("\n\n")
        .map(|g| g.chars().filter(|c| *c != '\n').collect::<HashSet<char>>())
        .map(|f| f.len())
        .sum::<usize>();

    println!("part 1: {:#?}", p1);

    let p2 = input
        .split("\n\n")
        .map(|g| g.lines().collect::<Vec<&str>>())
        .map(|g| count_all_answered(&g))
        .sum::<usize>();

    println!("part 2: {:#?}", p2);
}

fn count_all_answered(group: &Vec<&str>) -> usize {
    (b'a'..=b'z')
        .map(char::from)
        .filter(|l| {
            group
                .iter()
                .map(|g| g.chars().collect::<Vec<char>>())
                .all(|g| g.contains(l))
        })
        .collect::<Vec<char>>()
        .len()
}
