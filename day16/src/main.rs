use std::collections::HashSet;
use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

type Ticket = Vec<usize>;

fn main() {
    let input = include_str!("input");

    println!("Part 1: {}", pt1(input));
    println!("Part 2: {}", pt2(input));
}

fn pt1(input: &str) -> usize {
    let (fields, _, tickets) = parse_input(input);

    tickets
        .iter()
        .flat_map(|t| get_invalid_fields_on_ticket(t, &fields))
        .sum::<usize>()
}

fn pt2(input: &str) -> usize {
    let (fields, my_ticket, other_tickets) = parse_input(input);

    let mut field_possibilities: HashMap<usize, HashSet<&Field>> = HashMap::new();
    let mut field_impossibilites: HashMap<usize, HashSet<&Field>> = HashMap::new();

    for i in 0..my_ticket.len() {
        field_possibilities.insert(i, HashSet::new());
        field_impossibilites.insert(i, HashSet::new());
    }

    for valid_ticket in other_tickets
        .into_iter()
        .filter(|t| is_valid_ticket(t, &fields))
    {
        for (idx, item) in valid_ticket.iter().enumerate() {
            //find which field it could possibly be
            for field in fields.iter() {
                match field.contains(*item) {
                    true => field_possibilities.get_mut(&idx).unwrap().insert(field),
                    false => field_impossibilites.get_mut(&idx).unwrap().insert(field),
                };
            }
        }
    }
    for (idx, fields) in field_impossibilites.iter() {
        for f in fields {
            field_possibilities.get_mut(idx).unwrap().remove(f);
        }
    }

    let mut sol: Vec<(usize, &Field)> = Vec::new();

    while field_possibilities.len() > 0 {
        let solved = field_possibilities
            .clone()
            .into_iter()
            .filter(|(_, fields)| fields.len() == 1)
            .map(|(idx, hs)| (idx, *hs.iter().next().unwrap()))
            .map(|(idx, field)| {
                field_possibilities.remove(&idx);
                for (_, fields) in field_possibilities.iter_mut() {
                    fields.remove(field);
                }
                (idx, field)
            })
            .collect::<Vec<(usize, &Field)>>();

        for s in &solved {
            let found_field = fields.iter().find(|f| f.name == solved[0].1.name).unwrap();
            sol.push((s.0, found_field));
            // println!("SOLVED: idx {} ==> {}", s.0, found_field.name);
        }
    }

    sol.iter()
        .filter(|f| f.1.name.starts_with("departure"))
        .map(|(idx, _)| my_ticket[*idx])
        .product::<usize>()
}

fn parse_input(input: &str) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let mut input = input.split("\n\n");
    let fields = input
        .next()
        .unwrap()
        .lines()
        .map(|l| l.parse::<Field>().unwrap())
        .collect::<Vec<_>>();

    let my_ticket: Ticket = input.next().unwrap().lines().collect::<Vec<_>>()[1]
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    let other_tickets: Vec<Ticket> = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|i| i.parse().unwrap()).collect())
        .collect();

    return (fields, my_ticket, other_tickets);
}

fn is_valid_ticket(ticket: &Vec<usize>, fields: &Vec<Field>) -> bool {
    get_invalid_fields_on_ticket(ticket, fields).len() == 0
}

fn get_invalid_fields_on_ticket<'a>(
    ticket: &'a Vec<usize>,
    fields: &'a Vec<Field>,
) -> Vec<&'a usize> {
    ticket
        .iter()
        .filter(|&i| fields.iter().all(|f| !f.contains(*i)))
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Field {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp = s.split(":").map(|split| split.trim()).collect::<Vec<_>>();
        let ranges = sp[1]
            .split(" or ")
            .map(|ra| {
                let ra_split = ra
                    .split("-")
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                ra_split[0]..=ra_split[1]
            })
            .collect();
        return Ok(Field {
            name: String::from(sp[0]),
            ranges,
        });
    }
}

impl Field {
    fn contains(&self, i: usize) -> bool {
        self.ranges.iter().find(|r| r.contains(&i)).is_some()
    }
}
