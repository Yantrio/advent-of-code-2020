use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").expect("failed to read input file");
    let expenses = parse(&input).expect("Failed to parse input file");

    let p1 = find_result(&expenses, 2).unwrap();
    println!("Part 1 result : {}", p1);

    let p2 = find_result(&expenses, 3).unwrap();
    println!("Part 2 result : {}", p2);
}

fn parse(input: &str) -> Option<Vec<usize>> {
    input.lines().map(|s| s.parse().ok()).collect()
}

fn find_result(expenses: &Vec<usize>, amount_of_numbers: usize) -> usize {
    expenses
        .iter()
        .combinations(amount_of_numbers)
        .find(|c| c.iter().map(|&&x| x).sum::<usize>() == 2020)
        .map(|c| c.iter().map(|&&x| x).product())
        .unwrap()[0];
}

#[cfg(test)]
mod day1 {
    use super::find_result;

    #[test]
    fn part1() {
        let vec = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_result(&vec, 2).unwrap();
        assert_eq!(514579 as usize, result);
    }

    #[test]
    fn part2() {
        let vec = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_result(&vec, 3).unwrap();
        assert_eq!(241861950, result);
    }
}
