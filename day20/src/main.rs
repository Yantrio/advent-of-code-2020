use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<(), ()> {
    let input = include_str!("input");
    let tiles = get_tiles(input);
    let (corners, result) = part1(&tiles)?;

    println!("Part 1: {:?}", result);
    println!("Part 2: {:?}", part2(&tiles, &corners)?);

    Ok(())
}

fn part1(tiles: &Vec<Tile>) -> Result<(Vec<&Tile>, usize), ()> {
    let mut corners = Vec::new();

    for tile in tiles.iter() {
        let (mut n, mut s, mut e, mut w) = (false, false, false, false);
        for other in tiles.iter().filter(|t| t.id != tile.id) {
            match (
                tile.matches_north(other).is_some(),
                tile.matches_south(other).is_some(),
                tile.matches_east(other).is_some(),
                tile.matches_west(other).is_some(),
            ) {
                (true, _, _, _) => n = true,
                (_, true, _, _) => s = true,
                (_, _, true, _) => e = true,
                (_, _, _, true) => w = true,
                _ => {}
            }
        }
        match (n, s, e, w) {
            (false, true, true, false)
            | (false, true, false, true)
            | (true, false, true, false)
            | (true, false, false, true) => corners.push(tile),
            _ => {}
        };
    }
    Ok((
        corners.clone(),
        corners.iter().map(|c| c.id).product::<usize>(),
    ))
}

fn filter_tiles<'a>(tiles: &'a Vec<Tile>, unwanted_id: usize) -> impl Iterator<Item = &Tile> {
    tiles.into_iter().filter(move |t| t.id != unwanted_id)
}

fn part2(tiles: &Vec<Tile>, corners: &Vec<&Tile>) -> Result<usize, ()> {
    let grid = corners[0]
        .get_orientations()
        .iter()
        .filter(|o| filter_tiles(tiles, corners[0].id).any(|t| o.matches_south(t).is_some()))
        .filter(|o| filter_tiles(tiles, corners[0].id).any(|t| o.matches_west(t).is_some()))
        .map(|t| get_grid(tiles, t))
        .find(Result::is_ok)
        .map(|grid| grid.unwrap())
        .ok_or_else(|| ())?;

    Ok(find_rough_waters_with_monster(&mut grid_to_tile(&grid)?)?)
}

fn find_rough_waters_with_monster(grid: &mut Tile) -> Result<usize, ()> {
    // we're cheeky and use a tile so we can orient it easier using our methods from pt10
    let sea_monster = [
        "                  #",
        "#    ##    ##    ###",
        " #  #  #  #  #  #",
    ];

    let sea_monster_rules: Vec<_> = sea_monster
        .iter()
        .enumerate()
        .flat_map(|(x, &line)| {
            line.chars()
                .enumerate()
                .filter(move |(_, c)| '#' == *c)
                .map(move |(y, _)| (x, y))
        })
        .collect();

    for mut grid_orientation in grid.get_orientations() {
        let mut count = 0;

        for x in 0..(grid.pixels.len() - sea_monster[0].len()) {
            for y in 0..(grid.pixels.len() - sea_monster.len()) {
                let found_sea_monster = sea_monster_rules
                    .iter()
                    .all(|(dy, dx)| grid_orientation.pixels[y + dy][x + dx] == '#');

                if found_sea_monster {
                    for &(dy, dx) in sea_monster_rules.iter() {
                        grid_orientation.pixels[y + dy][x + dx] = 'O'
                    }
                    count += 1;
                }
            }
        }

        if count > 0 {
            return Ok(grid_orientation
                .pixels
                .iter()
                .map(|l| l.iter().filter(|&&c| c == '#').count())
                .sum::<usize>());
        }
    }
    Err(())
}

type Grid = HashMap<(usize, usize), Tile>;
fn get_grid(tiles: &Vec<Tile>, top_left: &Tile) -> Result<Grid, ()> {
    let mut grid = HashMap::new();
    grid.insert((0, 0), top_left.clone());

    let grid_size = (tiles.len() as f64).sqrt() as usize;

    // iterate columns and fill those out too!
    for x in 0..grid_size {
        if x != 0 {
            let up_left = grid.get(&(x - 1, 0)).ok_or_else(|| ())?;
            let found = filter_tiles(tiles, up_left.id)
                .map(|t| up_left.matches_west(t))
                .find(Option::is_some)
                .map(|t| t.unwrap())
                .ok_or_else(|| ())?;

            grid.insert((x, 0), found);
        }

        for y in 1..grid_size {
            let above = grid.get(&(x, y - 1)).ok_or_else(|| ())?;
            let found = filter_tiles(tiles, above.id)
                .map(|t| above.matches_south(t))
                .find(Option::is_some)
                .map(|t| t.unwrap())
                .ok_or_else(|| ())?;

            grid.insert((x, y), found);
        }
    }

    Ok(grid)
}

fn grid_to_tile(grid: &Grid) -> Result<Tile, ()> {
    let tile_size = grid.values().nth(0).ok_or_else(|| ())?.pixels.len() - 2;
    let grid_size = (grid.values().len() as f32).sqrt() as usize;
    let mut lines = (0..(grid_size * tile_size))
        .map(|_| "".to_string())
        .collect::<Vec<_>>();

    for y in 0..grid_size {
        for x in 0..grid_size {
            let current = grid.get(&(x, y)).ok_or_else(|| ())?;
            for (idx, st) in current.to_lines_without_borders().iter().enumerate() {
                lines[(y * tile_size) + idx] += st;
            }
        }
    }
    Ok(("Tile 0:\n".to_owned() + &lines.join("\n")).parse::<Tile>()?)
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
    fn get_borders(&self) -> Vec<String> {
        vec![
            self.pixels[0].iter().collect(),
            self.pixels[self.pixels.len() - 1].iter().collect(),
            self.pixels.iter().map(|p| p[0]).collect(),
            self.pixels.iter().map(|p| p[p.len() - 1]).collect(),
        ]
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
        let mut res = self.clone();
        let size = self.pixels.len();
        for i in 0..size {
            for j in 0..size {
                res.pixels[i][j] = self.pixels[size - j - 1][i]
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

    fn matches_north(&self, other: &Tile) -> Option<Tile> {
        self.matches_on_border(other, 0, 1)
    }

    fn matches_south(&self, other: &Tile) -> Option<Tile> {
        self.matches_on_border(other, 1, 0)
    }

    fn matches_west(&self, other: &Tile) -> Option<Tile> {
        self.matches_on_border(other, 3, 2)
    }

    fn matches_east(&self, other: &Tile) -> Option<Tile> {
        self.matches_on_border(other, 2, 3)
    }

    fn to_lines_without_borders(&self) -> Vec<String> {
        trim(&self.pixels)
            .map(trim)
            .map(Iterator::collect)
            .collect()
    }
}

fn trim<T: Clone>(input: &Vec<T>) -> impl Iterator<Item = &T> {
    input.iter().skip(1).take(input.len() - 2)
}
