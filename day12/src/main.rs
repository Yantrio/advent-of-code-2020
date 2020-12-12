fn main() {
    let input = include_str!("input");

    let directions = input
        .lines()
        .map(|l| l.split_at(1))
        .map(|(c, d)| (c, d.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&directions));
    println!("Part 2: {}", part2(&directions));
}

fn part1(instructions: &Vec<(&str, i64)>) -> i64 {
    let mut pos = (0, 0);
    let mut dir: (i64, i64) = (1, 0); // starts facing east
    for inst in instructions {
        let offset = inst.1;
        match inst.0 {
            "N" => pos.1 += inst.1,
            "S" => pos.1 -= inst.1,
            "E" => pos.0 += inst.1,
            "W" => pos.0 -= inst.1,
            "L" | "R" => dir = turn(&dir, &inst),
            "F" => {
                pos.0 += dir.0 * offset;
                pos.1 += dir.1 * offset;
            }
            _ => unreachable!(),
        };
    }

    pos.0.abs() + pos.1.abs()
}

fn part2(instructions: &Vec<(&str, i64)>) -> i64 {
    let mut pos = (0, 0);
    let mut wp = (10, 1);
    for inst in instructions {
        let offset = inst.1;
        match inst.0 {
            "N" => wp.1 += offset,
            "S" => wp.1 -= offset,
            "E" => wp.0 += offset,
            "W" => wp.0 -= offset,
            "L" | "R" => wp = turn(&wp, &inst),
            "F" => {
                pos.0 += offset * wp.0;
                pos.1 += offset * wp.1;
            }
            _ => unreachable!(),
        };
    }
    pos.0.abs() + pos.1.abs()
}

fn turn(facing: &(i64, i64), dir: &(&str, i64)) -> (i64, i64) {
    let x = facing.0 as f64;
    let y = facing.1 as f64;
    let mut deg = (dir.1 as f64).to_radians();
    if dir.0 == "R" {
        deg = deg * -1f64
    }
    let newx = (deg.cos() * x) - (deg.sin() * y);
    let newy = (deg.sin() * x) + (deg.cos() * y);

    (newx.round() as i64, newy.round() as i64)
}
