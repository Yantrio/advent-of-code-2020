use pathfinding::prelude::topological_sort;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Bag<'a> = (usize, &'a str);
type Bags<'a> = HashMap<&'a str, Vec<Bag<'a>>>;

fn main() {
    let input = include_str!("input");

    let bags = parse_input(input);

    println!("part 1: {}", part1(&bags));
    println!("part 2: {}", part2(&bags));
}

fn parse_input(input: &str) -> Bags {
    let re1 = Regex::new(r"^(.*) bags contain (.*)$").unwrap();
    let re2 = Regex::new(r"(\d+) (.*?) bag").unwrap();
    input
        .lines()
        .map(|l| {
            (
                re1.captures(l).unwrap().get(1).unwrap().as_str(),
                re2.captures_iter(l)
                    .map(|c| {
                        (
                            c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                            c.get(2).unwrap().as_str(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn topo_sort<'a>(bags: &'a Bags) -> Vec<&'a str> {
    let colors = &bags.keys().map(|&k| k).collect::<Vec<&str>>();
    topological_sort(colors, |bag_color| {
        (&bags[bag_color]).into_iter().map(|(_, c)| *c)
    })
    .unwrap()
    .into_iter()
    .rev()
    .collect()
}

fn part1(bags: &Bags) -> usize {
    let mut possible_bags = HashSet::new();
    possible_bags.insert("shiny gold");

    // because we've sorted topolgically, we just iterate back now
    for color in topo_sort(&bags) {
        if bags[color].iter().any(|(_, d)| possible_bags.contains(d)) {
            possible_bags.insert(color);
        }
    }

    possible_bags.len() - 1
}

fn part2(bags: &Bags) -> usize {
    let mut contents: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    for current_bag in topo_sort(bags) {
        let mut inner_contents = HashMap::new();
        for (amount, color) in bags[current_bag].iter() {
            // add if existing, if not existing add 0
            *inner_contents.entry(*color).or_insert(0) += amount;
            // dont forget the multiples! this is a tree not a straight graph
            for (&inner_color, multiple) in contents[color].iter() {
                *inner_contents.entry(&inner_color).or_insert(0) += amount * multiple;
            }
        }
        contents.insert(current_bag, inner_contents.into_iter().collect());
    }
    contents["shiny gold"]
        .iter()
        .map(|(_, &amount)| amount)
        .sum()
}
