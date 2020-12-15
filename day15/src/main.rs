fn main() {
    let input = include_str!("input")
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input, 2020));
    println!("Part 1: {}", part1(&input, 30000000));
}

fn part1(input: &Vec<usize>, rounds: usize) -> usize {
    let mut memory: Vec<SpokenNumber> = input
        .into_iter()
        .enumerate()
        .map(|(idx, i)| SpokenNumber::new(*i, idx))
        .collect();
    let mut last_spoken = memory.iter().last().unwrap().number;

    for idx in input.len()..rounds {
        if idx % 10000 == 0 {
            println!("{}/{}", idx, rounds);
        }
        last_spoken = step(&last_spoken, &mut memory, idx);
    }

    last_spoken
}

fn step(last_spoken: &usize, memory: &mut Vec<SpokenNumber>, idx: usize) -> usize {
    if let Some(found) = memory.iter_mut().find(|m| m.number == *last_spoken) {
        match found.spoken_indices.len() {
            1 => {
                speak(0, idx, memory);
                return 0;
            }
            _ => {
                let last_2 = found
                    .spoken_indices
                    .iter()
                    .rev()
                    .take(2)
                    .collect::<Vec<_>>();
                let diff = last_2[0] - last_2[1];

                speak(diff, idx, memory);
                return diff;
            }
        }
    }

    speak(0, idx, memory);
    return 0;
}

fn speak(number: usize, idx: usize, memory: &mut Vec<SpokenNumber>) {
    match memory.iter_mut().find(|m| m.number == number) {
        Some(diff) => {
            diff.spoken_indices.push(idx);
        }
        None => memory.push(SpokenNumber::new(number, idx)),
    }
}

#[derive(Debug, PartialEq)]
struct SpokenNumber {
    number: usize,
    spoken_indices: Vec<usize>,
}

impl SpokenNumber {
    fn new(number: usize, idx: usize) -> SpokenNumber {
        SpokenNumber {
            number,
            spoken_indices: vec![idx],
        }
    }
}
