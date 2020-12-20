use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("input");
    let tiles = get_tiles(input);
    let (corners, result) = part1(&tiles);

    println!("Part 1: {:?}", result);
    println!("Part 2: {:?}", part2(&tiles, &corners));
}

fn part1(tiles: &Vec<Tile>) -> (Vec<&Tile>, usize) {
    let mut corners = Vec::new();

    for tile in tiles.iter() {
        let (mut n, mut s, mut e, mut w) = (false, false, false, false);
        for comparison in tiles.iter().filter(|t| t.id != tile.id) {
            if !n && tile.matches_on_north(comparison).is_some() {
                n = true;
            }
            if !s && tile.matches_on_south(comparison).is_some() {
                s = true;
            }
            if !e && tile.matches_on_west(comparison).is_some() {
                e = true;
            }
            if !w && tile.matches_on_east(comparison).is_some() {
                w = true;
            }
        }
        match (n, s, e, w) {
            //TL, TR, BL, BR
            (false, true, true, false)
            | (false, true, false, true)
            | (true, false, true, false)
            | (true, false, false, true) => corners.push(tile),
            _ => {}
        };
    }
    (
        corners.clone(),
        corners.iter().map(|c| c.id).product::<usize>(),
    )
}

fn part2(tiles: &Vec<Tile>, corners: &Vec<&Tile>) -> usize {
    // solve left edge by getting top left corner first
    // find top left, then go down!
    // then go across going down each column in the same way

    let top_left = corners[0];

    let mut poss = vec![];
    for o in top_left.get_orientations().into_iter() {
        let match_down = tiles
            .iter()
            .filter(|t| t.id != top_left.id)
            .find(|t| o.matches_on_south(t).is_some());

        let match_right = tiles
            .iter()
            .filter(|t| t.id != top_left.id)
            .find(|t| o.matches_on_east(t).is_some());

        if match_down.is_some() && match_right.is_some() {
            poss.push(o);
        }
    }

    let grid = poss
        .iter()
        .map(|p| get_grid(tiles, &p))
        .find(Result::is_ok)
        .unwrap()
        .unwrap();

    find_rough_waters_with_monster(&mut grid_to_tile(&grid))
}

fn find_rough_waters_with_monster(grid: &mut Tile) -> usize {
    // we're cheeky and use a tile so we can orient it easier using our methods from pt1
    let sea_monster = [
        "                  #",
        "#    ##    ##    ###",
        " #  #  #  #  #  #",
    ];

    let sea_monster_rules: Vec<_> = sea_monster
        .iter()
        .enumerate()
        .flat_map(|(row, &line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c == '#' {
                    true => Some((row, col)),
                    false => None,
                })
        })
        .collect();

    for mut o in grid.get_orientations() {
        let mut count = 0;
        for y in 0..(grid.pixels.len() - sea_monster.len()) {
            for x in 0..(grid.pixels.len() - sea_monster[0].len()) {
                let found_sea_monster = sea_monster_rules
                    .iter()
                    .all(|(dy, dx)| o.pixels[y + dy][x + dx] == '#');

                if found_sea_monster {
                    for &(dy, dx) in sea_monster_rules.iter() {
                        o.pixels[y + dy][x + dx] = 'O'
                    }
                    count += 1;
                }
            }
        }

        if count > 0 {
            return o
                .pixels
                .iter()
                .map(|l| l.iter().filter(|&&c| c == '#').count())
                .sum::<usize>();
        }
    }
    unreachable!();
}

type Grid = HashMap<(usize, usize), Tile>;
fn get_grid(tiles: &Vec<Tile>, top_left: &Tile) -> Result<Grid, ()> {
    let all_tiles_but = |id: usize| tiles.into_iter().filter(move |t| t.id != id);

    let mut grid = HashMap::new();
    grid.insert((0, 0), top_left.clone());

    let grid_size = (tiles.len() as f64).sqrt() as usize;

    // iterate columns and fill those out too!
    for x in 0..grid_size {
        if x != 0 {
            let up_left = grid.get(&(x - 1, 0)).unwrap();
            let found = all_tiles_but(up_left.id)
                .map(|t| up_left.matches_on_east(t))
                .find(Option::is_some)
                .unwrap()
                .unwrap();

            grid.insert((x, 0), found);
        }

        for y in 1..grid_size {
            let above = grid.get(&(x, y - 1)).unwrap();
            let found = all_tiles_but(above.id)
                .map(|t| above.matches_on_south(t))
                .find(Option::is_some)
                .ok_or_else(|| ())?
                .ok_or_else(|| ())?;

            grid.insert((x, y), found);
        }
    }

    Ok(grid)
}

fn grid_to_tile(grid: &Grid) -> Tile {
    let tile_size = grid.values().nth(0).unwrap().pixels.len() - 2;
    let grid_size = (grid.values().len() as f32).sqrt() as usize;
    let mut lines = (0..(grid_size * tile_size))
        .map(|_| "".to_string())
        .collect::<Vec<_>>();

    for y in 0..grid_size {
        for x in 0..grid_size {
            let current = grid.get(&(x, y)).unwrap();
            for (idx, st) in current.to_lines_without_borders().iter().enumerate() {
                lines[(y * tile_size) + idx] += st;
            }
        }
    }
    ("Tile 0:\n".to_owned() + &lines.join("\n"))
        .parse::<Tile>()
        .unwrap()
}

fn get_tiles(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|t| t.parse::<Tile>().unwrap())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<char>>,
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let id_str = lines.next().unwrap().split("Tile ").nth(1).unwrap();

        Ok(Tile {
            id: id_str[0..id_str.len() - 1].parse::<usize>().unwrap(),
            pixels: lines.map(|l| l.chars().collect()).collect(),
        })
    }
}

impl Tile {
    fn get(&self, x: usize, y: usize) -> char {
        self.pixels[x][y]
    }

    fn set(&mut self, x: usize, y: usize, data: char) {
        self.pixels[x][y] = data;
    }

    fn get_borders(&self) -> Vec<String> {
        let mut res: Vec<String> = vec![
            self.pixels[0].iter().collect(),
            self.pixels[self.pixels.len() - 1].iter().collect(),
            self.pixels.iter().map(|p| p[0]).collect(),
            self.pixels.iter().map(|p| p[p.len() - 1]).collect(),
        ];

        // dont forget the mirrored borders
        res.append(
            &mut res
                .clone()
                .iter()
                .map(|i| i.chars().rev().collect::<String>())
                .collect::<Vec<_>>(),
        );

        res
    }

    fn get_orientations(&self) -> Vec<Tile> {
        let mut current: Tile = self.clone();
        let mut res: Vec<Tile> = Vec::new();
        for _ in 0..4 {
            res.push(current.clone());
            res.push(current.clone().flip_x());
            res.push(current.clone().flip_y());
            current = current.rotate_90();
        }

        res.into_iter().map(|r| r).collect()
    }

    fn rotate_90(&self) -> Tile {
        let mut res = Tile {
            id: self.id,
            pixels: self.pixels.clone(),
        };
        let size = self.pixels.len();
        for i in 0..size {
            for j in 0..size {
                res.set(i, j, self.get(size - j - 1, i));
            }
        }

        res
    }

    fn flip_y(&self) -> Tile {
        Tile {
            id: self.id,
            pixels: self.pixels.clone().into_iter().rev().collect(),
        }
    }

    fn flip_x(&self) -> Tile {
        Tile {
            id: self.id,
            pixels: self
                .pixels
                .clone()
                .into_iter()
                .map(|l| l.iter().rev().map(|&c| c).collect())
                .collect(),
        }
    }

    fn matches_on_border(&self, other: &Tile, idx: usize, other_idx: usize) -> Option<Tile> {
        let b = self.get_borders();
        for orientation in other.get_orientations().iter() {
            if b[idx] == orientation.get_borders()[other_idx] {
                return Some(orientation.clone());
            }
        }
        None
    }

    fn matches_on_north(&self, other: &Tile) -> Option<Tile> {
        self.matches_on_border(other, 0, 1)
    }

    fn matches_on_south(&self, other: &Tile) -> Option<Tile> {
        self.matches_on_border(other, 1, 0)
    }

    fn matches_on_east(&self, other: &Tile) -> Option<Tile> {
        self.matches_on_border(other, 3, 2)
    }

    fn matches_on_west(&self, other: &Tile) -> Option<Tile> {
        self.matches_on_border(other, 2, 3)
    }

    fn to_lines_without_borders(&self) -> Vec<String> {
        self.pixels
            .iter()
            .skip(1)
            .take(self.pixels.len() - 2)
            .map(|line| line.iter().skip(1).take(line.len() - 2).collect::<String>())
            .collect::<Vec<_>>()
    }
}
