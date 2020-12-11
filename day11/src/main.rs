use array2d::Array2D;

fn main() {
    let input = include_str!("input");

    println!("Part 1: {}", pt1(input));
    println!("Part 2: {}", pt2(input));
}

fn solve(input: &str, step: &dyn Fn(&Array2D<char>) -> Array2D<char>) -> usize {
    let mut current = Array2D::from_rows(
        &input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<char>>>(),
    );

    let mut prev_occupied = count_occupied(&current);
    let mut current_occupied = usize::MAX;

    while prev_occupied != current_occupied {
        prev_occupied = current_occupied;
        current = step(&current);
        current_occupied = count_occupied(&current);
    }
    current_occupied
}

fn pt1(input: &str) -> usize {
    solve(input, &step)
}

fn pt2(input: &str) -> usize {
    solve(input, &step2)
}

fn count_occupied(input: &Array2D<char>) -> usize {
    let mut c = 0;

    for row_iter in input.rows_iter() {
        c += row_iter.into_iter().filter(|&&c| c == '#').count();
    }
    c
}

fn step(input: &Array2D<char>) -> Array2D<char> {
    let mut next_gen = input.clone();

    for row in 0i64..input.num_rows() as i64 {
        for col in 0i64..input.num_columns() as i64 {
            let occupied_adj = get_adj(&input, row, col)
                .iter()
                .filter(|&&&c| c == '#')
                .count();
            let new_val = match (input.get(row as usize, col as usize).unwrap(), occupied_adj) {
                ('L', 0) => '#',
                ('#', a) => match a >= 4 {
                    true => 'L',
                    false => '#',
                },
                (c, _) => *c,
            };

            next_gen.set(row as usize, col as usize, new_val).unwrap();
        }
    }
    next_gen
}

fn step2(input: &Array2D<char>) -> Array2D<char> {
    let mut next_gen = input.clone();

    for row in 0i64..input.num_rows() as i64 {
        for col in 0i64..input.num_columns() as i64 {
            let occupied_in_los = get_occupied_in_los(input, row, col);

            let new_val = match (
                input.get(row as usize, col as usize).unwrap(),
                occupied_in_los,
            ) {
                ('L', 0) => '#',
                ('#', a) => match a >= 5 {
                    true => 'L',
                    false => '#',
                },
                (&c, _) => c,
            };

            next_gen.set(row as usize, col as usize, new_val).unwrap();
        }
    }
    next_gen
}

const DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_occupied_in_los(input: &Array2D<char>, row: i64, col: i64) -> usize {
    let mut count = 0;
    'direction: for dir in DIRECTIONS.iter() {
        let mut distance = 0;
        loop {
            distance += 1;
            let pos = (row + (dir.0 * distance), col + (dir.1 * distance));
            match input.get(pos.0 as usize, pos.1 as usize) {
                Some(e) => match e {
                    '#' => {
                        count += 1;
                        continue 'direction;
                    }
                    'L' => {
                        continue 'direction;
                    }
                    _ => {}
                },
                None => continue 'direction,
            }
        }
    }

    count
}

fn get_adj(input: &Array2D<char>, row: i64, col: i64) -> Vec<&char> {
    DIRECTIONS
        .iter()
        .map(|(dx, dy)| (row + dx, col + dy))
        .filter(|(ax, ay)| *ax >= 0 && *ay >= 0)
        .map(|(ax, ay)| input.get(ax as usize, ay as usize))
        .filter(|f| f.is_some())
        .map(|f| f.unwrap())
        .collect::<Vec<&char>>()
}
