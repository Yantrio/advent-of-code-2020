use std::{collections::HashMap, unreachable};

fn main() {
    let mut input = include_str!("input").split("\n\n");

    let mut rules = parse_rules(input.next().unwrap());
    let strings = input.next().unwrap().lines().collect::<Vec<_>>();

    println!("Part 1: {}", solve(&rules, &strings));

    rules.insert(8, Rule::parse("8: 42 | 42 8").1);
    rules.insert(11, Rule::parse("11: 42 31 | 42 11 31").1);

    println!("Part 2: {}", solve(&rules, &strings));
}

fn solve(rules: &HashMap<usize, Rule>, strings: &Vec<&str>) -> usize {
    let rule_0 = rules.get(&0).unwrap();

    let strings = strings
        .iter()
        .map(|s| (rule_0.matches(&rules, s), s))
        .collect::<Vec<_>>();

    let complete_matches = strings
        .iter()
        .map(|(match_result, _)| match_result.iter().find(|m| m.len() == 0))
        .filter(Option::is_some)
        .collect::<Vec<_>>();

    complete_matches.len()
}

#[derive(Debug, PartialEq)]
enum Rule {
    Char(char),
    Ref(usize),
    Or(Box<Rule>, Box<Rule>),
    And2(usize, usize),
    And3(usize, usize, usize),
}

impl Rule {
    fn parse(s: &str) -> (usize, Rule) {
        let mut rule_id = 0;
        let mut raw_rule = s;
        if s.contains(":") {
            let sp = s.split(": ").collect::<Vec<_>>();
            rule_id = sp[0].parse().unwrap();
            raw_rule = sp[1];
        }

        if raw_rule.starts_with('"') {
            return (rule_id, Rule::Char(raw_rule.chars().nth(1).unwrap()));
        } else if raw_rule.contains(" | ") {
            let split = raw_rule.split(" | ").collect::<Vec<_>>();
            return (
                rule_id,
                Rule::Or(
                    Box::new(Rule::parse(split[0]).1),
                    Box::new(Rule::parse(split[1]).1),
                ),
            );
        } else if raw_rule.contains(' ') {
            let parts = raw_rule
                .split_ascii_whitespace()
                .map(|r| r.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            return (
                rule_id,
                match parts.len() {
                    3 => Rule::And3(parts[0], parts[1], parts[2]),
                    2 => Rule::And2(parts[0], parts[1]),
                    _ => unreachable!(),
                },
            );
        }
        return (rule_id, Rule::Ref(raw_rule.parse::<usize>().unwrap()));
    }

    fn matches<'a>(&self, all_rules: &HashMap<usize, Rule>, input: &'a str) -> Vec<&'a str> {
        if input.is_empty() {
            return vec![];
        }

        match self {
            Rule::Ref(r) => all_rules.get(r).unwrap().matches(all_rules, input),
            Rule::Char(value) => match input.chars().nth(0).unwrap() == *value {
                true => vec![&input[1..]],
                false => vec![],
            },
            Rule::Or(l, r) => {
                let mut res = Vec::new();
                res.append(&mut l.matches(all_rules, input));
                res.append(&mut r.matches(all_rules, input));
                res
            }
            Rule::And2(a, b) => {
                let mut res = Vec::new();
                for i in all_rules.get(a).unwrap().matches(all_rules, input) {
                    for j in all_rules.get(b).unwrap().matches(all_rules, i) {
                        res.push(j);
                    }
                }
                res
            }
            Rule::And3(a, b, c) => {
                let mut res = Vec::new();
                for i in all_rules.get(a).unwrap().matches(all_rules, input) {
                    for j in all_rules.get(b).unwrap().matches(all_rules, i) {
                        for k in all_rules.get(c).unwrap().matches(all_rules, j) {
                            res.push(k);
                        }
                    }
                }
                res
            }
        }
    }
}

fn parse_rules(input: &str) -> HashMap<usize, Rule> {
    input.lines().map(|r| Rule::parse(r)).collect()
}

#[cfg(test)]
mod day19 {
    use super::parse_rules;
    use super::Rule;
    use std::collections::HashMap;

    fn test(rules: &HashMap<usize, Rule>, idx: usize, str: &str) -> bool {
        let res = rules.get(&idx).unwrap().matches(rules, str);
        println!("INPUT: {}, RES: {:?}, len: {}", str, res, res.len());
        match res.iter().find(|r| r.len() == 0) {
            Some(_) => true,
            None => false,
        }
    }

    #[test]
    fn example_pt1_1() {
        let rules = parse_rules("0: 1 2\n1: \"a\"\n2: 1 3 | 3 1\n3: \"b\"");

        assert_eq!(test(&rules, 0, "aab"), true);
        assert_eq!(test(&rules, 0, "aba"), true);
        assert_eq!(test(&rules, 0, "baa"), false);
    }

    #[test]
    fn example_pt1_2() {
        let rules = parse_rules(
            r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"#,
        );
        let should_pass = vec!["aabaab"];

        for t in should_pass {
            println!("TESTING: {}", t);
            assert_eq!(test(&rules, 0, t), true);
        }
    }

    #[test]
    fn example_pt2() {
        let rule_string = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"#;

        let tests = [
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];

        let mut rules = parse_rules(rule_string);

        rules.insert(8, Rule::parse("8: 42 | 42 8").1);
        rules.insert(11, Rule::parse("11: 42 31 | 42 11 31").1);

        let passed = tests.iter().map(|t| test(&rules, 0, t)).filter(|&p| p);

        assert_eq!(passed.count(), 12)
    }

    #[test]
    fn and3_test() {
        let rules = parse_rules("0: 1 1 1\n1: \"a\"");
        assert_eq!(test(&rules, 0, "aaa"), true);
        assert_eq!(test(&rules, 0, "aba"), false);
        assert_eq!(test(&rules, 0, "aab"), false);
        assert_eq!(test(&rules, 0, "baa"), false);
        assert_eq!(test(&rules, 0, "aa"), false);
        assert_eq!(test(&rules, 0, "aaaa"), false);
        assert_eq!(test(&rules, 0, "aaab"), false);
    }

    #[test]
    fn and2_test() {
        let rules = parse_rules("0: 1 1\n1: \"a\"");
        assert_eq!(test(&rules, 0, "aa"), true);
        assert_eq!(test(&rules, 0, "ab"), false);
        assert_eq!(test(&rules, 0, "ba"), false);
        assert_eq!(test(&rules, 0, "aaa"), false);
        assert_eq!(test(&rules, 0, "aab"), false);
        assert_eq!(test(&rules, 0, "a"), false);
        assert_eq!(test(&rules, 0, "b"), false);
    }

    #[test]
    fn or_test() {
        let rules = parse_rules("0: 1 1 | 2 2\n1: \"a\"\n2: \"b\"");
        // assert_eq!(test(&rules, 0, "aa"), true);
        assert_eq!(test(&rules, 0, "bb"), true);
        assert_eq!(test(&rules, 0, "ab"), false);
        assert_eq!(test(&rules, 0, "ba"), false);
        assert_eq!(test(&rules, 0, "aaa"), false);
        assert_eq!(test(&rules, 0, "aab"), false);
        assert_eq!(test(&rules, 0, "a"), false);
        assert_eq!(test(&rules, 0, "b"), false);
    }

    #[test]
    fn should_parse_char_rule() {
        let rule = "0: \"a\"";
        let (id, rule) = Rule::parse(rule);
        assert_eq!(id, 0);
        assert_eq!(rule, Rule::Char('a'))
    }

    #[test]
    fn should_parse_ref_rule() {
        let rule = "0: 2";
        let (id, rule) = Rule::parse(rule);
        assert_eq!(id, 0);
        assert_eq!(rule, Rule::Ref(2))
    }

    #[test]
    fn should_parse_or_rule() {
        let rule = "0: 1 2 | 2 1";
        let (id, rule) = Rule::parse(rule);
        assert_eq!(id, 0);
        assert_eq!(
            rule,
            Rule::Or(Box::new(Rule::And2(1, 2)), Box::new(Rule::And2(2, 1)))
        )
    }

    #[test]
    fn should_parse_and_rule() {
        let rule = "0: 1 2";
        let (id, rule) = Rule::parse(rule);
        assert_eq!(id, 0);
        assert_eq!(rule, Rule::And2(1, 2));
    }

    #[test]
    fn should_parse_and3_rule() {
        let rule = "0: 1 2 3";
        let (id, rule) = Rule::parse(rule);
        assert_eq!(id, 0);
        assert_eq!(rule, Rule::And3(1, 2, 3));
    }

    #[test]
    fn should_parse_combination_rule() {
        let rule = "0: 1 2 | 2 1";
        let (id, rule) = Rule::parse(rule);
        assert_eq!(id, 0);
        assert_eq!(
            rule,
            Rule::Or(Box::new(Rule::And2(1, 2)), Box::new(Rule::And2(2, 1)))
        )
    }
}
