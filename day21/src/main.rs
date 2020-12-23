use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

fn main() -> Result<(), ()> {
    let input = include_str!("input");

    let foods = input
        .lines()
        .map(|l| l.parse::<Food>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&foods));
    println!("Part 2: {}", part2(&foods));
    Ok(())
}

fn part1(foods: &Vec<Food>) -> usize {
    let solved_allergens = solve_allergens(&foods);

    let known_ing = solved_allergens
        .iter()
        .map(|s| s.0.clone())
        .collect::<Vec<_>>();

    foods
        .iter()
        .flat_map(|f| f.ingredients.clone())
        .filter(|i| !known_ing.contains(i))
        .count()
}

fn part2(foods: &Vec<Food>) -> String {
    let mut solved_allergens = solve_allergens(&foods);
    solved_allergens.sort_by_key(|(_, a)| a.clone());

    solved_allergens
        .iter()
        .map(|(i, _)| i.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn solve_allergens(foods: &Vec<Food>) -> Vec<(String, String)> {
    let possible_allergens = get_all_possible_allergens(foods);

    let mut possible_allergens_for_ingredient: HashMap<String, HashSet<String>> = HashMap::new();
    // for each allergen
    for allergen in possible_allergens.keys() {
        //get all foods with this allergen
        let foods_with_all = foods
            .iter()
            .filter(|f| f.allergens.contains(allergen))
            .collect::<Vec<_>>();

        for ing in foods_with_all.iter().flat_map(|f| f.ingredients.clone()) {
            if foods_with_all.iter().all(|f| f.ingredients.contains(&ing)) {
                match possible_allergens_for_ingredient.get_mut(&ing) {
                    Some(v) => {
                        v.insert(allergen.clone());
                    }
                    None => {
                        possible_allergens_for_ingredient
                            .insert(ing, HashSet::from_iter(vec![allergen.clone()]));
                    }
                }
            }
        }
    }

    let mut found = vec![];

    loop {
        let pairs = possible_allergens_for_ingredient.clone();
        let (k, v) = pairs.iter().find(|(_, v)| v.len() == 1).unwrap();
        let (found_ing, found_all) = (k, v.iter().next().unwrap());
        found.push((found_ing.clone(), found_all.clone()));

        // pull that ingredient
        possible_allergens_for_ingredient.remove(k);
        for (_, v) in possible_allergens_for_ingredient.iter_mut() {
            v.remove(&found_all.clone());
        }

        if possible_allergens_for_ingredient.len() == 0 {
            break;
        }
    }

    found
}

fn get_all_possible_allergens(foods: &Vec<Food>) -> HashMap<String, Vec<String>> {
    let mut possible_allergens: HashMap<String, Vec<String>> = HashMap::new();

    for f in foods.iter() {
        for i in f.ingredients.iter() {
            for a in f.allergens.iter() {
                match possible_allergens.get_mut(a) {
                    Some(i_list) => {
                        i_list.push(i.clone());
                    }
                    None => {
                        possible_allergens.insert(a.clone(), vec![i.clone()]);
                    }
                }
            }
        }
    }

    possible_allergens
}

#[derive(Debug, PartialEq)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split(" (contains");
        let ingredients = sp
            .next()
            .ok_or_else(|| ())?
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect();
        let mut full_allergens = sp.next().ok_or_else(|| ())?;
        full_allergens = &full_allergens[..full_allergens.len() - 1].trim();
        let allergens = full_allergens.split(", ").map(|s| s.to_owned()).collect();

        Ok(Food {
            ingredients,
            allergens,
        })
    }
}
