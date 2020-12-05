fn main() {
    let input = include_str!("input");
    let passes = input
        .lines()
        .map(get_seat_details)
        .map(|p| p.2)
        .collect::<Vec<usize>>();

    let max_seat_id = passes.iter().max().unwrap();

    println!("part 1 {:#?}", max_seat_id);

    let empty_seats = (0..*max_seat_id)
        .into_iter()
        .filter(|sid| !passes.contains(sid))
        .collect::<Vec<usize>>();

    // be a human and read the output, find the number not at the start or end!
    println!("Part 2 {:#?}", empty_seats);
}

fn get_seat_details(input: &str) -> (usize, usize, usize) {
    let row = wittle_down(input.chars().take(7).collect(), 128);
    let col = wittle_down(input.chars().skip(7).collect(), 8);
    (row, col, row * 8 + col)
}

fn wittle_down(input: Vec<char>, max: usize) -> usize {
    let mut current_slice: &[usize] = &(0..max).collect::<Vec<usize>>();
    input.iter().enumerate().for_each(|(i, c)| {
        current_slice = &current_slice
            .chunks(max / (2 << i))
            .nth(match c {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => 0,
            })
            .unwrap();
    });
    current_slice[0]
}

#[cfg(test)]
mod day5 {
    use super::get_seat_details;

    #[test]
    fn part1_examples() {
        assert_eq!((70, 7, 567), get_seat_details("BFFFBBFRRR"));
        assert_eq!((14, 7, 119), get_seat_details("FFFBBBFRRR"));
        assert_eq!((102, 4, 820), get_seat_details("BBFFBBFRLL"));
    }
}
