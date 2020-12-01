use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").expect("failed to read input file");
    let expenses = parse(&input).expect("Failed to parse input file");

    let p1 = pt1(&expenses).unwrap();
    println!("Part 1 result : {}", p1);

    let p2 = pt2(&expenses).unwrap();
    println!("Part 2 result : {}", p2);
}

fn parse(input: &str) -> Option<Vec<usize>> {
    input.lines().map(|s| s.parse().ok()).collect()
}

fn pt1<'a>(expenses: &'a Vec<usize>) -> Result<usize, &'static str> {
    for e1 in expenses.iter() {
        for e2 in expenses.iter() {
            if e1 + e2 == 2020 {
                return Ok(e1 * e2);
            }
        }
    }
    return Err("Failed to find two numbers to add up to 2020");
}

fn pt2<'a>(expenses: &'a Vec<usize>) -> Result<usize, &'static str> {
    for e1 in expenses.iter() {
        for e2 in expenses.iter() {
            for e3 in expenses.iter() {
                if e1 + e2 + e3 == 2020 {
                    return Ok(e1 * e2 * e3);
                }
            }
        }
    }
    return Err("Failed to find three numbers to add up to 2020");
}

#[cfg(test)]
mod day1 {
    use super::pt1;
    use super::pt2;

    #[test]
    fn part1() {
        let vec = vec![1721, 979, 366, 299, 675, 1456];
        let result = pt1(&vec).unwrap();
        assert_eq!(514579 as usize, result);
    }

    #[test]
    fn part2() {
        let vec = vec![1721, 979, 366, 299, 675, 1456];
        let result = pt2(&vec).unwrap();
        assert_eq!(241861950, result);
    }
}
