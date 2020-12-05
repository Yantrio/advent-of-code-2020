fn main() {
    let input = include_str!("input");
    let passes = input
        .lines()
        .map(get_seat_details)
        .map(|(r, c)| r * 8 + c)
        .collect::<Vec<usize>>();

    let max_seat_id = passes.iter().max().unwrap();
    let min_seat_id = passes.iter().min().unwrap();

    let empty_seats = (*min_seat_id..*max_seat_id)
        .into_iter()
        .filter(|sid| !passes.contains(sid))
        .collect::<Vec<usize>>()[0];

    println!("Part 1 {:#?}", max_seat_id);
    println!("Part 2 {:#?}", empty_seats);
}

fn get_seat_details(input: &str) -> (usize, usize) {
    (wittle_down(&input[..7]), wittle_down(&input[7..]))
}

fn wittle_down(input: &str) -> usize {
    // we know the size based on the input, its 2^(len(input)-1)
    let mut current_slice: &[usize] = &(0..(2 << input.len() - 1)).collect::<Vec<usize>>();
    // avoid recursion, its not needed
    for c in input.chars() {
        current_slice = &current_slice
            .chunks(current_slice.len() / 2)
            .nth(match c {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => unreachable!(),
            })
            .unwrap();
    }
    current_slice[0]
}

#[cfg(test)]
mod day5 {
    use super::get_seat_details;

    #[test]
    fn part1_examples() {
        assert_eq!((70, 7), get_seat_details("BFFFBBFRRR"));
        assert_eq!((14, 7), get_seat_details("FFFBBBFRRR"));
        assert_eq!((102, 4), get_seat_details("BBFFBBFRLL"));
    }
}
