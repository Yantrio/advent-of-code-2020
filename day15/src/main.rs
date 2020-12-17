use std::collections::HashMap;

fn main() {
    let input = include_str!("input")
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", solve(&input, 2020));
    println!("Part 2: {}", solve(&input, 30000000));
}

fn solve(input: &Vec<usize>, rounds: usize) -> usize {
    let mut spoken: HashMap<usize, usize> = HashMap::new();

    for (idx, num) in input.iter().enumerate() {
        spoken.insert(*num, idx);
    }

    let mut last_spoken = *input.iter().last().unwrap();

    for idx in spoken.len()..rounds {
        let current = match spoken.get(&last_spoken) {
            None => 0,
            Some(last_spoken_idx) => idx - last_spoken_idx - 1,
        };

        spoken.insert(last_spoken, idx - 1);
        last_spoken = current;
    }
    last_spoken
}
