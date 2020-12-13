use ring_algorithm::chinese_remainder_theorem;
fn main() {
    let input = include_str!("input");
    let parsed = parse_input(&input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn parse_input(input: &str) -> (usize, Vec<(usize, usize)>) {
    let lines = input.lines().collect::<Vec<_>>();
    return (
        lines[0].parse::<usize>().unwrap(),
        lines[1]
            .split(',')
            .enumerate()
            .filter(|&(_, b)| b != "x")
            .map(|(i, b)| (i, b.parse::<usize>().unwrap()))
            .collect::<Vec<_>>(),
    );
}

fn get_lowest_multiple_after_ts(timestamp: &usize, bus: &usize) -> usize {
    bus * ((timestamp + (bus - 1)) / bus)
}

fn part1(input: &(usize, Vec<(usize, usize)>)) -> usize {
    let (ts, busses) = input;

    let earliest_bus = busses
        .iter()
        .map(|&(idx, bus)| (idx, bus, get_lowest_multiple_after_ts(&ts, &bus)))
        .min_by_key(|&b| b.2)
        .unwrap();

    // bus number * (lowest_mult_after_ts - ts)
    earliest_bus.1 * (earliest_bus.2 - ts)
}

fn part2(input: &(usize, Vec<(usize, usize)>)) -> usize {
    let (_, busses) = input;

    let (u, m): (Vec<_>, Vec<_>) = busses
        .iter()
        .map(|&(i, bus)| (-(i as i64), bus as i64))
        .unzip();

    chinese_remainder_theorem(&u, &m).unwrap() as usize
}
