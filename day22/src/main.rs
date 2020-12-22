use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input");
    let hands = parse_hands(input);

    println!("Part 1: {}", pt1(hands.clone()));
    println!("Part 2: {:?}", &pt2(hands.clone()).1);
}

fn pt1(hands: (VecDeque<usize>, VecDeque<usize>)) -> usize {
    let (mut p1, mut p2) = hands;

    loop {
        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();
        match p1_card > p2_card {
            true => {
                p1.push_back(p1_card);
                p1.push_back(p2_card);
            }
            false => {
                p2.push_back(p2_card);
                p2.push_back(p1_card);
            }
        }

        if p1.is_empty() || p2.is_empty() {
            break;
        }
    }

    get_score(&p1).max(get_score(&p2))
}

fn pt2(hands: (VecDeque<usize>, VecDeque<usize>)) -> (usize, usize) {
    let (mut p1, mut p2) = hands;

    let mut previous_rounds: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();

    loop {
        //before starting the round, if we've already done this combination in this game, p1 wins
        if previous_rounds.contains(&(p1.clone(), p2.clone())) {
            return (1, get_score(&p1.clone()));
        }
        previous_rounds.insert((p1.clone(), p2.clone()));

        // draw top cards
        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();

        let winner = match (
            p1.len() >= p1_card && p2.len() >= p2_card,
            p1_card > p2_card,
        ) {
            (true, _) => {
                // RECURSIVE COMBAT
                pt2((
                    p1.clone().into_iter().take(p1_card).collect(),
                    p2.clone().into_iter().take(p2_card).collect(),
                ))
                .0
            }
            (false, true) => 1,
            (false, false) => 2,
        };

        match winner {
            1 => {
                p1.push_back(p1_card);
                p1.push_back(p2_card);
            }
            2 => {
                p2.push_back(p2_card);
                p2.push_back(p1_card);
            }
            _ => unreachable!(),
        }

        if p1.is_empty() {
            return (2, get_score(&p2.clone()));
        }
        if p2.is_empty() {
            return (1, get_score(&p1.clone()));
        }
    }
}

fn get_score(hand: &VecDeque<usize>) -> usize {
    hand.clone()
        .make_contiguous()
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| (idx + 1) * val)
        .sum()
}

fn parse_hands(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let players = input.split("\n\n").collect::<Vec<_>>();

    let get_cards = |s: &str| {
        s.lines()
            .skip(1)
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>()
    };

    (get_cards(players[0]), get_cards(players[1]))
}
