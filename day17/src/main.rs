use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

fn main() {
    let input = include_str!("input");

    println!("Part 1: {}", solve(input, 3));
    println!("Part 2: {}", solve(input, 4));
}

fn solve(input: &str, dimensions: usize) -> usize {
    let mut grid: Grid = input.parse().unwrap();
    for _ in 0..6 {
        grid = grid.step(dimensions);
    }
    grid.count_active()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    fn new(w: isize, x: isize, y: isize, z: isize) -> Position {
        Position { w, x, y, z }
    }
}

impl Position {
    fn iterate_neighbours(
        &self,
        dimensions: usize,
        f: &mut dyn FnMut((isize, isize, isize, isize)),
    ) {
        let get_range = |dim_depth: usize| match dimensions >= dim_depth {
            true => -1..=1,
            false => 0..=0,
        };

        for dz in get_range(1) {
            for dy in get_range(2) {
                for dx in get_range(3) {
                    for dw in get_range(4) {
                        f((dw, dx, dy, dz));
                    }
                }
            }
        }
    }

    fn get_neighbouring_positions(&self, dimensions: usize) -> Vec<Position> {
        let mut result = vec![];

        self.iterate_neighbours(dimensions, &mut |(dw, dx, dy, dz)| match (dw, dx, dy, dz) {
            (0, 0, 0, 0) => {}
            _ => {
                result.push(Position {
                    w: self.w + dw,
                    x: self.x + dx,
                    y: self.y + dy,
                    z: self.z + dz,
                });
            }
        });

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
        for (y, line) in s.lines().enumerate().map(|(y, line)| (y as isize, line)) {
            for (x, c) in line.chars().enumerate().map(|(x, c)| (x as isize, c)) {
                let is_active = match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                };
                cubes.insert(Position::new(0, x, y, 0), is_active);
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
        let get_range =
            |input: Vec<isize>| *input.iter().min().unwrap() - 1..=*input.iter().max().unwrap() + 1;
        let w_range = get_range(self.cubes.keys().map(|c| c.w).collect());
        let x_range = get_range(self.cubes.keys().map(|c| c.x).collect());
        let y_range = get_range(self.cubes.keys().map(|c| c.y).collect());
        let z_range = get_range(self.cubes.keys().map(|c| c.z).collect());

        return (w_range, x_range, y_range, z_range);
    }

    fn count_active(&self) -> usize {
        self.cubes.values().filter(|&&v| v).count()
    }

    fn step(&self, dimensions: usize) -> Grid {
        let mut new_grid = Grid::new();

        let b = self.get_bounds();
        for z in b.3.clone() {
            for y in b.2.clone() {
                for x in b.1.clone() {
                    for w in if dimensions == 4 { b.0.clone() } else { 0..=0 } {
                        let pos = Position { x, y, z, w };
                        let is_active = self.cubes.get(&pos).unwrap_or(&false);

                        let active_neighbour_count = pos
                            .get_neighbouring_positions(dimensions)
                            .iter()
                            .map(|p| self.cubes.get(&p))
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
}
