use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

fn main() {
    let mut grid: Grid = include_str!("input").parse().unwrap();

    for _ in 0..6 {
        grid = step_pt1(&grid);
        // grid.print();
    }
    println!("Part 1: {}", grid.count_active());
}

fn step_pt1(grid: &Grid) -> Grid {
    let mut new_grid = Grid::new();

    let bounds = grid.get_bounds();

    let new_bounds = (
        bounds.0.start() - 1..=bounds.0.end() + 1,
        bounds.1.start() - 1..=bounds.1.end() + 1,
        bounds.2.start() - 1..=bounds.2.end() + 1,
    );
    // iterate all

    for z in new_bounds.2.clone() {
        for y in new_bounds.1.clone() {
            for x in new_bounds.0.clone() {
                let pos = Position { x, y, z };
                let is_active = grid.cubes.get(&pos).unwrap_or(&false);

                let active_neighbour_count = pos
                    .get_neighbouring_positions()
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

    new_grid
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    fn get_neighbouring_positions(&self) -> impl Iterator<Item = Position> + '_ {
        [
            (-1, -1, -1),
            (-1, -1, 0),
            (-1, -1, 1),
            (-1, 0, -1),
            (-1, 0, 0),
            (-1, 0, 1),
            (-1, 1, -1),
            (-1, 1, 0),
            (-1, 1, 1),
            (0, -1, -1),
            (0, -1, 0),
            (0, -1, 1),
            (0, 0, -1),
            (0, 0, 1),
            (0, 1, -1),
            (0, 1, 0),
            (0, 1, 1),
            (1, -1, -1),
            (1, -1, 0),
            (1, -1, 1),
            (1, 0, -1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, -1),
            (1, 1, 0),
            (1, 1, 1),
        ]
        .iter()
        .map(move |neigh| Position {
            x: self.x + neigh.0,
            y: self.y + neigh.1,
            z: self.z + neigh.2,
        })
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

impl Grid {
    fn new() -> Grid {
        Grid {
            cubes: HashMap::new(),
        }
    }

    fn get_bounds(
        &self,
    ) -> (
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
    ) {
        let max_x = self.cubes.iter().map(|(p, _)| p.x).max().unwrap();
        let max_y = self.cubes.iter().map(|(p, _)| p.y).max().unwrap();
        let max_z = self.cubes.iter().map(|(p, _)| p.z).max().unwrap();

        let min_x = self.cubes.iter().map(|(p, _)| p.x).min().unwrap();
        let min_y = self.cubes.iter().map(|(p, _)| p.y).min().unwrap();
        let min_z = self.cubes.iter().map(|(p, _)| p.z).min().unwrap();

        return (min_x..=max_x, min_y..=max_y, min_z..=max_z);
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
                let c = match self.cubes.get(&Position { x, y, z }) {
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
