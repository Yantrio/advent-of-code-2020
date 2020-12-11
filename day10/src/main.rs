#![feature(array_windows)]

fn main() {
    let input = include_str!("input");

    println!("Part 1: {}", part1(&mut parse(input)));
    println!("Part 2: {}", part2(&mut parse(input)));
}

fn parse(input: &str) -> Vec<usize> {
    let mut adapters: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort_unstable();
    if let Some(&max) = adapters.last() {
        adapters.push(max + 3);
    };

    adapters
}

fn part1(adapters: &mut Vec<usize>) -> usize {
    let diffs = adapters
        .windows(2)
        .map(|win| win[1] - win[0])
        .collect::<Vec<usize>>();

    let count_diffs = |diff| diffs.iter().filter(|&&a| a == diff).count();

    count_diffs(1) * count_diffs(3)
}

fn part2(adapters: &mut Vec<usize>) -> usize {
    let get_path_count = |u: &[&[usize]]| match u.len() {
        // count numbers of paths in the graph based on the amount of matches
        4 => 7,
        3 => 4,
        2 => 2,
        _ => 1,
    };
    // the number of possible paths is equal to the product of the number of paths from each node in the graph

    adapters
        .windows(2) //iterate pairs
        .collect::<Vec<_>>()
        .split(|&n| n[1] - n[0] == 3) // grab all items up to a diff of 3
        .map(get_path_count)
        .product::<usize>()
}
