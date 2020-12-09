use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    println!("Part 1 {:#?}", part1(input));
    println!("Part 2 {:#?}", part2(input));
}

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1(input: &str) -> usize {
    let data = parse(input);

    let preamble_size = 25;
    *data
        .iter()
        .enumerate()
        .skip(preamble_size)
        .find(|(idx, _)| find_pair_for_item(&data, *idx, preamble_size).is_none())
        .map(|(_, number)| number)
        .unwrap()
}

fn part2(input: &str) -> usize {
    let items = &parse(input);
    let target_number = part1(input);
    let get_range = |start, range_size| items.iter().skip(start).take(range_size).map(|&s| s);

    for start in 1..items.len() {
        for range_size in 2..items.len() - start {
            let current_range = get_range(start, range_size);
            match current_range.sum::<usize>() == target_number {
                true => {
                    let range = get_range(start, range_size).collect::<Vec<usize>>();
                    return *range.iter().min().unwrap() + *range.iter().max().unwrap();
                }
                false => {}
            };
        }
    }

    unreachable!();
}

fn find_pair_for_item(items: &Vec<usize>, idx: usize, preamble_size: usize) -> Option<Vec<&usize>> {
    if idx < preamble_size {
        return None;
    }
    items
        .iter()
        .skip(idx - preamble_size)
        .take(preamble_size)
        .combinations(2)
        .find(|c| c.iter().map(|&&x| x).sum::<usize>() == *items.get(idx).unwrap())
}
