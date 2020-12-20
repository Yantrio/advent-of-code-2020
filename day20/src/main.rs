use std::str::FromStr;

fn main() {
    let input = include_str!("input");
    let tiles = get_tiles(input);
    let (corners, result) = part1(&tiles);

    println!("Corners: {:?}", result);

    part2(&tiles, &corners);
}

fn part1(tiles: &Vec<Tile>) -> (Vec<&Tile>, usize) {
    let mut corners = Vec::new();

    for tile in tiles.iter() {
        let (mut n, mut s, mut e, mut w) = (false, false, false, false);
        for comparison in tiles.iter().filter(|t| t.id != tile.id) {
            if !n && tile.matches_on_north(comparison) {
                n = true;
            }
            if !s && tile.matches_on_south(comparison) {
                s = true;
            }
            if !e && tile.matches_on_west(comparison) {
                e = true;
            }
            if !w && tile.matches_on_east(comparison) {
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

fn part2(tiles: &Vec<Tile>, corners: &Vec<&Tile>) {
    //solve left edge by getting top left corner first
    //find top left, then go down!
}

fn get_tiles(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|t| t.parse::<Tile>().unwrap())
        .collect()
}

fn intersection<'a>(l: &'a Vec<String>, r: &'a Vec<String>) -> Vec<String> {
    let mut common = Vec::new();
    for item in l.iter() {
        if r.contains(&item) {
            common.push(item.clone());
        }
    }

    common
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
        let id = &id_str[0..id_str.len() - 1].parse::<usize>().unwrap();

        Ok(Tile {
            id: *id,
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

        for i in res.clone().iter() {
            let reverse = i.chars().rev().collect();
            res.push(reverse);
        }
        res
    }

    fn get_orientations(&self) -> Vec<Tile> {
        let mut current: Tile = self.clone();
        let mut res: Vec<Tile> = Vec::new();
        for _ in 0..4 {
            for f in current.get_flips() {
                res.push(f);
            }
            current = current.rotate_90();
        }

        res.into_iter().map(|r| r).collect()
    }

    fn get_flips(&self) -> Vec<Tile> {
        let mut res: Vec<Tile> = Vec::new();

        res.push(self.clone());
        res.push(self.clone().flip());

        res
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

    fn flip(&self) -> Tile {
        Tile {
            id: self.id,
            pixels: self.pixels.clone().into_iter().rev().collect(),
        }
    }

    fn matches_on_border(&self, other: &Tile, idx: usize, other_idx: usize) -> bool {
        let b = self.get_borders();
        let mut found = false;
        for orientation in other.get_orientations().iter() {
            if b[idx] == orientation.get_borders()[other_idx] {
                found = true;
                break;
            }
        }
        found
    }

    fn matches_on_north(&self, other: &Tile) -> bool {
        self.matches_on_border(other, 0, 1)
    }

    fn matches_on_south(&self, other: &Tile) -> bool {
        self.matches_on_border(other, 1, 0)
    }

    fn matches_on_east(&self, other: &Tile) -> bool {
        self.matches_on_border(other, 3, 2)
    }
    fn matches_on_west(&self, other: &Tile) -> bool {
        self.matches_on_border(other, 2, 3)
    }
}

#[cfg(test)]
mod day20 {

    use super::Tile;

    #[test]
    fn should_parse_tile() {
        let input = r#"Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;

        let parsed: Tile = input.parse().unwrap();

        assert_eq!(parsed.id, 3079);
        assert_eq!(parsed.pixels.len(), 10);
        assert_eq!(parsed.pixels[0].len(), 10);
        assert_eq!(parsed.pixels[0], "#.#.#####.".chars().collect::<Vec<_>>());
    }

    #[test]
    fn should_get_correct_borders() {
        let input = r#"Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;
        let parsed: Tile = input.parse().unwrap();
        let borders = parsed.get_borders();
        let expected = vec!["#.#.#####.", "..#.###...", "#..##.#...", ".#....#..."]
            .into_iter()
            .map(|b| {
                let rev = b.clone().chars().rev().collect::<String>();
                (b, rev)
            })
            .flat_map(|(b, r)| vec![b.chars().collect(), r])
            .collect::<Vec<String>>();
        for b in borders {
            assert_eq!(expected.contains(&b), true);
        }
    }

    #[test]
    fn should_rotate() {
        let input = r#"Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;

        let expected = vec![
            "...#.##..#",
            "....###.#.",
            "####.###.#",
            "...#.##...",
            "#.##..#.##",
            "#.#####.##",
            "#.##....##",
            "....#...##",
            "...###..##",
            "...#....#.",
        ]
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

        let parsed: Tile = input.parse().unwrap();
        let rotated = parsed.rotate_90();

        assert_eq!(rotated.id, parsed.id);
        assert_eq!(rotated.pixels, expected);
    }

    #[test]
    fn should_flip_y() {
        let input = r#"Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;

        let expected = vec![
            "..#.###...",
            "..#.......",
            "..#.###...",
            "#.#####.##",
            ".#...#.##.",
            "####.#..#.",
            "######....",
            "..#.......",
            ".#..######",
            "#.#.#####.",
        ]
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

        let parsed: Tile = input.parse().unwrap();
        let flipped = parsed.flip();
        println!(
            "{:#?}",
            flipped
                .pixels
                .iter()
                .map(|p| p.iter().collect::<String>())
                .collect::<Vec<_>>()
        );

        assert_eq!(flipped.id, parsed.id);
        assert_eq!(flipped.pixels, expected);
    }

    #[test]
    fn get_orientations() {
        let input = r#"Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###..."#;
        let parsed: Tile = input.parse().unwrap();

        let orientations = parsed.get_orientations();

        assert_eq!(orientations.len(), 8);
    }

    #[test]
    fn matches_on_north() {
        let a = "Tile 0:\n##\n..".parse::<Tile>().unwrap();
        let b = "Tile 1:\n..\n##".parse::<Tile>().unwrap();
        assert_eq!(a.matches_on_north(&b), true);
    }

    #[test]
    fn matches_on_south() {
        let a = "Tile 0:\n##\n..".parse::<Tile>().unwrap();
        let b = "Tile 1:\n..\n.a".parse::<Tile>().unwrap();
        assert_eq!(a.matches_on_south(&b), true);
    }
    #[test]
    fn matches_on_east() {
        let a = "Tile 0:\n##\n..".parse::<Tile>().unwrap();
        let b = "Tile 1:\n#a\n.a".parse::<Tile>().unwrap();
        assert_eq!(a.matches_on_east(&b), true);
    }

    #[test]
    fn matches_on_west() {
        let a = "Tile 0:\n#.\n#.".parse::<Tile>().unwrap();
        let b = "Tile 1:\n.#\n.#".parse::<Tile>().unwrap();
        assert_eq!(a.matches_on_west(&b), true);
    }
}
