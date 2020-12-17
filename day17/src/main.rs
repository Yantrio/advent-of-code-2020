use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

fn main() {
    let input = include_str!("input");

    println!("Part 1: {}", solve(input, 3));
    println!("Part 1: {}", solve(input, 4));
}

fn solve(input: &str, dimensions: usize) -> usize {
    let mut grid: Grid = input.parse().unwrap();

    for i in 0..6 {
        grid = step_pt1(&grid, dimensions);
        // grid.print();
        // println!("{} {}", i, grid.count_active())
    }
    grid.count_active()
}

fn step_pt1(grid: &Grid, dimensions: usize) -> Grid {
    let mut new_grid = Grid::new();

    let b = grid.get_bounds();
    for z in b.3.clone() {
        for y in b.2.clone() {
            for x in b.1.clone() {
                for w in if dimensions == 4 { b.0.clone() } else { 0..=0 } {
                    let pos = Position { x, y, z, w };
                    let is_active = grid.cubes.get(&pos).unwrap_or(&false);

                    let active_neighbour_count = pos
                        .get_neighbouring_positions(dimensions)
                        .iter()
                        .map(|p| grid.cubes.get(&p))
                        .filter(|&n| *n.unwrap_or(&false))
                        .count();

                    let will_be_active = match (is_active, active_neighbour_count) {
                        (true, 2) | (true, 3) | (false, 3) => true,
                        _ => false,
                    };

                    new_grid.cubes.insert(pos.clone(), will_be_active);
                }
            }
        }
    }

    new_grid
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    fn get_neighbouring_positions(&self, dimensions: usize) -> Vec<Position> {
        let mut result = vec![];

        for z in if dimensions >= 1 { -1..=1 } else { 0..=0 } {
            for y in if dimensions >= 2 { -1..=1 } else { 0..=0 } {
                for x in if dimensions >= 3 { -1..=1 } else { 0..=0 } {
                    for w in if dimensions >= 4 { -1..=1 } else { 0..=0 } {
                        match (w, x, y, z) {
                            (0, 0, 0, 0) => {}
                            _ => {
                                result.push(Position {
                                    w: self.w + w,
                                    x: self.x + x,
                                    y: self.y + y,
                                    z: self.z + z,
                                });
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

#[derive(Debug)]
struct Grid {
    cubes: HashMap<Position, bool>,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes: HashMap<Position, bool> = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                cubes.insert(
                    Position {
                        w: 0,
                        x: x as isize,
                        y: y as isize,
                        z: 0,
                    },
                    match c {
                        '#' => true,
                        '.' => false,
                        _ => unreachable!(),
                    },
                );
            }
        }

        Ok(Grid { cubes })
    }
}

type Bounds = (
    RangeInclusive<isize>,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
);

impl Grid {
    fn new() -> Grid {
        Grid {
            cubes: HashMap::new(),
        }
    }

    fn get_bounds(&self) -> Bounds {
        let max_w = self.cubes.iter().map(|(p, _)| p.w).max().unwrap() + 1;
        let max_x = self.cubes.iter().map(|(p, _)| p.x).max().unwrap() + 1;
        let max_y = self.cubes.iter().map(|(p, _)| p.y).max().unwrap() + 1;
        let max_z = self.cubes.iter().map(|(p, _)| p.z).max().unwrap() + 1;

        let min_w = self.cubes.iter().map(|(p, _)| p.w).min().unwrap() - 1;
        let min_x = self.cubes.iter().map(|(p, _)| p.x).min().unwrap() - 1;
        let min_y = self.cubes.iter().map(|(p, _)| p.y).min().unwrap() - 1;
        let min_z = self.cubes.iter().map(|(p, _)| p.z).min().unwrap() - 1;

        return (min_w..=max_w, min_x..=max_x, min_y..=max_y, min_z..=max_z);
    }

    fn print_layer(&self, z: isize) {
        let on_layer = self.cubes.iter().filter(|c| c.0.z == z).collect::<Vec<_>>();

        if on_layer.len() == 0 {
            return;
        }

        let max_x = on_layer.iter().max_by_key(|p| p.0.x).unwrap().0.x;
        let max_y = on_layer.iter().max_by_key(|p| p.0.y).unwrap().0.y;

        let min_x = on_layer.iter().min_by_key(|p| p.0.x).unwrap().0.x;
        let min_y = on_layer.iter().min_by_key(|p| p.0.y).unwrap().0.y;

        for y in min_y..=max_y {
            let mut line = vec![];
            for x in min_x..=max_x {
                let c = match self.cubes.get(&Position { x, y, z, w: 0 }) {
                    Some(v) => match v {
                        true => '#',
                        false => '.',
                    },
                    None => '.',
                };
                line.push(c);
            }
            println!("{}", line.into_iter().collect::<String>())
        }
    }

    fn print(&self) {
        for z in self.get_bounds().2 {
            println!("z={}", z);
            self.print_layer(z);
            println!("");
        }
    }

    fn count_active(&self) -> usize {
        self.cubes.values().filter(|&&v| v).count()
    }
}
