use std::collections::HashMap;

use indicatif::ProgressBar;

fn main() {
    let input = "219347865";
    let labels = &mut input
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&mut parse(labels), labels[0]));

    for i in *labels.iter().max().unwrap()..1_000_000 {
        labels.push(i);
    }

    println!("len {}", labels.len());

    println!("Part 2: {}", part2(&mut parse(labels), labels[0]));
}

fn part1(cups: &mut HashMap<usize, usize>, mut current_cup: usize) -> String {
    for _ in 0..100 {
        current_cup = step(cups, current_cup);
    }

    cups_to_vec(&cups, 1)
        .iter()
        .rev()
        .skip(1)
        .rev()
        .map(|c| c.to_string())
        .collect()
}

fn part2(cups: &mut HashMap<usize, usize>, mut current_cup: usize) -> String {
    let bar = ProgressBar::new(10_000_000);
    for i in 1..=10_000_000 {
        if i % 10_000 == 0 {
            bar.inc(1000);
        }
        current_cup = step(cups, current_cup);
    }
    bar.finish();

    let first = cups.get(&1).unwrap();
    let second = cups.get(&first).unwrap();

    println!("1. {}, 2. {}", first, second);

    (first * second).to_string()
}

fn parse(input: &Vec<usize>) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    for p in input.windows(2) {
        res.insert(p[0], p[1]);
    }
    res.insert(input[input.len() - 1], input[0]); //make it circular

    res
}

fn cups_to_vec(cups: &HashMap<usize, usize>, start: usize) -> Vec<usize> {
    let mut current_cup = start;
    let mut res = vec![];
    loop {
        current_cup = *cups.get(&current_cup).unwrap();
        res.push(current_cup);
        if current_cup == start {
            break;
        }
    }

    res
}

fn step(cups: &mut HashMap<usize, usize>, current_cup: usize) -> usize {
    let taken = take_3(cups, current_cup);
    let mut destination = current_cup - 1;

    loop {
        if destination == 0 {
            destination = cups.len();
            continue;
        }
        match taken.contains(&destination) {
            false => {
                break;
            }
            true => {
                destination -= 1;
            }
        }
    }
    insert(cups, taken, destination);
    *cups.get(&current_cup).unwrap()
}

fn take_3(cups: &mut HashMap<usize, usize>, current_cup: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut cup = current_cup;
    for _ in 0..3 {
        cup = *cups.get(&cup).unwrap();
        res.push(cup);
    }

    // join the gap
    cups.insert(current_cup, *cups.get(&cup).unwrap());

    res
}

fn insert(cups: &mut HashMap<usize, usize>, to_insert: Vec<usize>, destination: usize) {
    let prev_ptr = *cups.get(&destination).unwrap();
    cups.insert(destination, to_insert[0]);
    cups.insert(to_insert[0], to_insert[1]);
    cups.insert(to_insert[1], to_insert[2]);
    cups.insert(to_insert[2], prev_ptr);
}

#[cfg(test)]
mod day23 {

    use super::insert;
    use super::parse;
    use super::take_3;
    use crate::HashMap;

    #[test]
    fn should_parse() {
        let res = parse(&vec![1, 2, 3, 4, 5]);
        let expected = [(1, 2), (2, 3), (3, 4), (4, 5), (5, 1)]
            .iter()
            .map(|&(a, b)| (a as usize, b as usize))
            .collect::<HashMap<usize, usize>>();

        assert_eq!(res, expected);
    }

    #[test]
    fn should_take_3() {
        let cups = parse(&vec![1, 2, 3, 4, 5]);

        assert_eq!(take_3(&mut cups.clone(), 1), vec![2, 3, 4]);
        assert_eq!(take_3(&mut cups.clone(), 2), vec![3, 4, 5]);
        assert_eq!(take_3(&mut cups.clone(), 3), vec![4, 5, 1]);
        assert_eq!(take_3(&mut cups.clone(), 4), vec![5, 1, 2]);
        assert_eq!(take_3(&mut cups.clone(), 5), vec![1, 2, 3]);
    }

    #[test]
    fn should_take_3_and_update_cups() {
        let mut cups = parse(&vec![1, 2, 3, 4, 5]);

        take_3(&mut cups, 1);

        assert_eq!(*cups.get(&1).unwrap(), 5 as usize);
        assert_eq!(*cups.get(&5).unwrap(), 1 as usize);
    }

    #[test]
    fn insert_should_work() {
        let mut cups = parse(&vec![1, 5]);

        insert(&mut cups, vec![2, 3, 4], 1);

        let expected = [(1, 2), (2, 3), (3, 4), (4, 5), (5, 1)]
            .iter()
            .map(|&(a, b)| (a as usize, b as usize))
            .collect::<HashMap<usize, usize>>();

        assert_eq!(cups, expected);
    }
}
