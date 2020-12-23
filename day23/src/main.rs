use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::collections::VecDeque;

fn main() {
    let mut input = "219347865";

    println!("Part 1: {}", pt1(input));

    input = "219347865"; // example input;
    println!("Part 2: {}", pt2(input));
}

fn pt1(input: &str) -> String {
    let mut c = input
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect::<VecDeque<usize>>();

    c.reserve(c.len() + 10);
    let cups = run_steps(&mut c, 100);

    let res = cups
        .iter()
        .skip(1)
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("");

    res.to_string()
}

fn pt2(input: &str) -> String {
    let mut cups = input
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect::<VecDeque<usize>>();

    cups.reserve(cups.len() + 10);

    let max_cup = *cups.iter().max().unwrap();
    for i in max_cup..1_000_000 {
        cups.push_back(i);
    }

    let cups = run_steps(&mut cups, 10_000_000);

    let res = cups
        .iter()
        .skip(1)
        .take(2)
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",");

    res.to_string()
}

fn run_steps<'a>(cups: &'a mut VecDeque<usize>, steps: usize) -> &'a VecDeque<usize> {
    let mut current_cup = 0;

    let bar = ProgressBar::new(steps as u64);

    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} {per_sec}")
            .progress_chars("##-"),
    );

    for _ in 0..steps {
        bar.inc(1);
        current_cup = step(cups, current_cup);
    }
    bar.finish();

    //get pos of 1
    let idx = cups.iter().position(|&c| c == 1).unwrap();
    cups.rotate_left(idx);

    cups
}

fn step(cups: &mut VecDeque<usize>, mut ccup_idx: usize) -> usize {
    let current_cup = cups.get(ccup_idx).unwrap().clone();

    let taken = take_3(cups, ccup_idx);

    let mut destination = current_cup - 1;
    while !cups.contains(&destination) {
        destination = match destination == 0 {
            true => *cups.iter().max().unwrap(),
            false => destination - 1,
        }
    }

    //find the index of the destination
    let dest_idx = cups.iter().position(|&c| c == destination).unwrap() + 1;

    cups.insert(dest_idx % (cups.len() + 1), taken[2]);
    cups.insert(dest_idx % (cups.len() + 1), taken[1]);
    cups.insert(dest_idx % (cups.len() + 1), taken[0]);

    ccup_idx = cups.iter().position(|&c| c == current_cup).unwrap();

    (ccup_idx + 1) % cups.len()
}

fn take_3(cups: &mut VecDeque<usize>, ccup_idx: usize) -> Vec<usize> {
    let mut res = vec![];
    for _ in 0..3 {
        let mut idx_to_remove = ccup_idx + 1;
        if idx_to_remove >= cups.len() {
            idx_to_remove = 0;
        }
        res.push(cups.remove(idx_to_remove).unwrap());
    }
    res
}

#[cfg(test)]
mod day23 {

    use super::take_3;
    use std::collections::VecDeque;

    fn get_cups() -> VecDeque<usize> {
        vec![1, 2, 3, 4, 5]
            .iter()
            .map(|&i| i as usize)
            .collect::<VecDeque<usize>>()
    }
    #[test]
    fn take3() {
        assert_eq!(take_3(&mut get_cups(), 0), vec![2, 3, 4]);
        assert_eq!(take_3(&mut get_cups(), 1), vec![3, 4, 5]);
        assert_eq!(take_3(&mut get_cups(), 2), vec![4, 5, 1]);
        assert_eq!(take_3(&mut get_cups(), 3), vec![5, 1, 2]);
        assert_eq!(take_3(&mut get_cups(), 4), vec![1, 2, 3]);
    }
}
