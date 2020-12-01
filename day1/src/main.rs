use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").expect("failed to read input file");
    let expenses = parse(&input).expect("Failed to parse input file");

    let p1 = pt1(&expenses);
    println!("Part 1 result : {}", p1);

    let p2 = pt2(&expenses);
    println!("Part 2 result : {}", p2);
}

fn parse(input: &str) -> Option<Vec<usize>> {
    input.lines().map(|s| s.parse().ok()).collect()
}

fn pt1<'a>(expenses: &'a Vec<usize>) -> usize {
    let result = expenses
        .iter()
        .tuple_combinations()
        .find(|&(a, b)| a + b == 2020)
        .unwrap();

    return result.0 * result.1;
}

fn pt2<'a>(expenses: &'a Vec<usize>) -> usize {
    let result = expenses
        .iter()
        .tuple_combinations()
        .find(|&(a, b, c)| a + b + c == 2020)
        .unwrap();

    return result.0 * result.1 * result.2;
}

#[cfg(test)]
mod day1 {
    use super::pt1;
    use super::pt2;

    #[test]
    fn part1() {
        let vec = vec![1721, 979, 366, 299, 675, 1456];
        let result = pt1(&vec);
        assert_eq!(514579 as usize, result);
    }

    #[test]
    fn part2() {
        let vec = vec![1721, 979, 366, 299, 675, 1456];
        let result = pt2(&vec);
        assert_eq!(241861950, result);
    }
}
