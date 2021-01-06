use std::collections::HashMap;

type Position = (i32, i32, i32);

type Map = HashMap<Position, bool>;

fn main() {
    let input = include_str!("input");

    let mut map = handle_paths(input);
    println!(
        "Part 1: {}", // count the black tiles
        map.values().filter(|&&v| v == true).count()
    );

    for i in 0..100 {
        map = end_of_day(&mut map);
        println!(
            "Day {}: {}",
            i + 1,
            map.values().filter(|&&v| v == true).count()
        );
    }

    println!("Part 2: {}", map.values().filter(|&&v| v == true).count())
}

fn handle_paths(input: &str) -> Map {
    let paths = input.lines().map(|l| parse_line(l)).collect::<Vec<_>>();
    let mut map: Map = Map::new();

    for path in paths.iter() {
        //step all the way along, flip the tile
        let mut pos = (0, 0, 0);
        for d in path {
            pos = step(d, pos);
        }
        let tile = map.entry(pos).or_insert(false);
        *tile = !*tile
    }

    map
}

fn end_of_day(map: &mut HashMap<(i32, i32, i32), bool>) -> Map {
    let mut m = map.clone();

    let all_positions = map.keys();

    let x_range = (all_positions.clone().min_by_key(|p| p.0).unwrap().0 - 2)
        ..(all_positions.clone().max_by_key(|p| p.0).unwrap().0 + 2);

    let y_range = (all_positions.clone().min_by_key(|p| p.1).unwrap().1 - 2)
        ..(all_positions.clone().max_by_key(|p| p.1).unwrap().1 + 2);

    let z_range = (all_positions.clone().min_by_key(|p| p.2).unwrap().2 - 2)
        ..(all_positions.clone().max_by_key(|p| p.2).unwrap().2 + 2);

    for x in x_range.clone() {
        for y in y_range.clone() {
            for z in z_range.clone() {
                let current_pos = (x, y, z);
                let mut surrounding_black = 0;
                for pos in get_surrounding_positions(current_pos) {
                    if let Some(black) = map.get(&pos) {
                        if *black {
                            surrounding_black += 1;
                        }
                    }
                }

                let current_tile_color = *map.entry(current_pos).or_insert(false);
                if current_tile_color == true && (surrounding_black == 0 || surrounding_black > 2) {
                    m.insert(current_pos, false);
                }
                if current_tile_color == false && surrounding_black == 2 {
                    m.insert(current_pos, true);
                }
            }
        }
    }

    m
}

fn get_surrounding_positions(pos: Position) -> Vec<Position> {
    [
        Direction::East,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
        Direction::NorthEast,
    ]
    .iter()
    .map(|d| step(d, pos))
    .collect::<Vec<_>>()
}

fn step(dir: &Direction, pos: Position) -> Position {
    let (x, y, z) = pos;
    match dir {
        Direction::East => (x + 1, y - 1, z),
        Direction::SouthEast => (x, y - 1, z + 1),
        Direction::SouthWest => (x - 1, y, z + 1),
        Direction::West => (x - 1, y + 1, z),
        Direction::NorthWest => (x, y + 1, z - 1),
        Direction::NorthEast => (x + 1, y, z - 1),
    }
}

fn parse_line(input: &str) -> Vec<Direction> {
    let mut idx: usize = 0;
    let mut result = Vec::new();

    let chars = input.chars().collect::<Vec<_>>();

    while idx < input.len() {
        match chars[idx] {
            'e' => {
                result.push(Direction::East);
                idx += 1;
            }
            'w' => {
                result.push(Direction::West);
                idx += 1;
            }
            'n' => match chars[idx + 1] {
                'e' => {
                    result.push(Direction::NorthEast);
                    idx += 2;
                }
                'w' => {
                    result.push(Direction::NorthWest);
                    idx += 2;
                }
                _ => unreachable!(),
            },
            's' => match chars[idx + 1] {
                'e' => {
                    result.push(Direction::SouthEast);
                    idx += 2;
                }
                'w' => {
                    result.push(Direction::SouthWest);
                    idx += 2;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    result
}

enum Direction {
    East,
    NorthEast,
    SouthEast,
    West,
    NorthWest,
    SouthWest,
}
