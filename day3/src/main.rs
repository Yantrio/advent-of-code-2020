use array2d::Array2D;

fn main() {
    let input = include_str!("input");

    let landscape = parse_input(input);

    println!("part 1 {:#?}", trees_on_slope(&landscape, 3, 1));

    // part2
    let slope_results: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|slope| trees_on_slope(&landscape, slope.0, slope.1))
        .product();

    println!("part 2{:#?}", slope_results);
}

fn parse_input(input: &str) -> Array2D<char> {
    Array2D::from_rows(
        &input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    )
}

fn trees_on_slope(landscape: &Array2D<char>, right: usize, down: usize) -> usize {
    (0..landscape.num_rows() / down)
        .map(|y| (y * down, (y * right).rem_euclid(landscape.num_columns())))
        .map(|loc| landscape.get(loc.0, loc.1).unwrap())
        .filter(|&&c| c == '#')
        .count()
}

#[cfg(test)]
mod day1 {
    use super::parse_input;
    use super::trees_on_slope;

    #[test]
    fn example_part1() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let landscape = parse_input(input);

        let result = trees_on_slope(&landscape, 3, 1);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_part2() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let landscape = parse_input(input);
        let slope_results: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|slope| trees_on_slope(&landscape, slope.0, slope.1))
            .product();
        assert_eq!(336, slope_results);
    }
}
