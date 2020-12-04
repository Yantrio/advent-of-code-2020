use regex::Regex;

fn main() {
    let input = include_str!("input");

    let passports = input
        .split("\n\n")
        .map(|p| p.lines().collect())
        .map(Passport::new)
        .collect::<Vec<Passport>>();

    println!(
        "Part 1 {:#?}",
        passports.iter().filter(|p| p.has_required_fields()).count()
    );

    println!(
        "Part 2 {:#?}",
        passports
            .iter()
            .filter(|p| p.has_required_fields())
            .filter(|p| p.is_valid())
            .count()
    );
}

#[derive(Copy, Clone, Debug)]
struct Passport<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    expiration_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn new(lines: Vec<&'a str>) -> Passport<'a> {
        let props = lines
            .iter()
            .flat_map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<&str>>();

        return Passport {
            birth_year: get_value("byr", &props),
            issue_year: get_value("iyr", &props),
            expiration_year: get_value("eyr", &props),
            height: get_value("hgt", &props),
            hair_color: get_value("hcl", &props),
            eye_color: get_value("ecl", &props),
            passport_id: get_value("pid", &props),
            country_id: get_value("cid", &props),
        };
    }

    fn has_required_fields(self) -> bool {
        [
            self.birth_year,
            self.issue_year,
            self.expiration_year,
            self.height,
            self.hair_color,
            self.eye_color,
            self.passport_id,
        ]
        .iter()
        .all(Option::is_some)
    }

    fn is_valid(self) -> bool {
        Passport::is_valid_year(self.birth_year.unwrap(), 1920, 2002)
            && Passport::is_valid_year(self.issue_year.unwrap(), 2010, 2020)
            && Passport::is_valid_year(self.expiration_year.unwrap(), 2020, 2030)
            && Passport::is_valid_height(self.height.unwrap())
            && Passport::is_valid_hair_color(self.hair_color.unwrap())
            && Passport::is_valid_eye_color(self.eye_color.unwrap())
            && Passport::is_valid_pid(self.passport_id.unwrap())
    }

    fn is_valid_hair_color(input: &str) -> bool {
        let re = Regex::new("^#[a-f0-9]{6}$").unwrap();
        re.is_match(input)
    }

    fn is_valid_height(input: &str) -> bool {
        if !input.ends_with("cm") && !input.ends_with("in") {
            return false;
        }

        let v = input
            .replace("cm", "")
            .replace("in", "")
            .parse::<usize>()
            .unwrap();

        match (input.ends_with("cm"), input.ends_with("in")) {
            (true, false) => is_inbetween_inclusive(v, 150, 193),
            (false, true) => is_inbetween_inclusive(v, 59, 76),
            _ => false,
        }
    }

    fn is_valid_eye_color(input: &str) -> bool {
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input)
    }

    fn is_valid_pid(input: &str) -> bool {
        Regex::new("^[\\d]{9}$").unwrap().is_match(input)
    }

    fn is_valid_year(input: &str, min: usize, max: usize) -> bool {
        match input.parse::<usize>() {
            Ok(val) => is_inbetween_inclusive(val, min, max),
            Err(_) => false,
        }
    }
}

fn get_value<'a>(key: &str, props: &Vec<&'a str>) -> Option<&'a str> {
    match props.iter().find(|&&p| p.starts_with(key)) {
        None => None,
        Some(&a) => a.split(':').nth(1),
    }
}

fn is_inbetween_inclusive(num: usize, min: usize, max: usize) -> bool {
    num >= min && num <= max
}

#[cfg(test)]
mod day4 {
    use super::Passport;

    #[test]
    fn is_valid_year() {
        assert_eq!(true, Passport::is_valid_year("1999", 1920, 2002));
        assert_eq!(true, Passport::is_valid_year("1920", 1920, 2002));
        assert_eq!(true, Passport::is_valid_year("2002", 1920, 2002));

        assert_eq!(false, Passport::is_valid_year("1919", 1920, 2002));
        assert_eq!(false, Passport::is_valid_year("2003", 1920, 2002));
        assert_eq!(false, Passport::is_valid_year("199", 1920, 2002));
        assert_eq!(false, Passport::is_valid_year("abcs", 1920, 2002));
        assert_eq!(false, Passport::is_valid_year("22222", 1920, 2002));
    }

    #[test]
    fn is_valid_height() {
        assert_eq!(true, Passport::is_valid_height("150cm"));
        assert_eq!(true, Passport::is_valid_height("160cm"));
        assert_eq!(true, Passport::is_valid_height("193cm"));

        assert_eq!(true, Passport::is_valid_height("59in"));
        assert_eq!(true, Passport::is_valid_height("76in"));
        assert_eq!(true, Passport::is_valid_height("61in"));

        assert_eq!(false, Passport::is_valid_height("149cm"));
        assert_eq!(false, Passport::is_valid_height("1600cm"));
        assert_eq!(false, Passport::is_valid_height("194cm"));

        assert_eq!(false, Passport::is_valid_height("58in"));
        assert_eq!(false, Passport::is_valid_height("7600in"));
        assert_eq!(false, Passport::is_valid_height("77in"));

        assert_eq!(false, Passport::is_valid_height("190"));
        assert_eq!(false, Passport::is_valid_height("190in"));
    }

    #[test]
    fn is_valid_hair_color() {
        assert_eq!(true, Passport::is_valid_hair_color("#111111"));
        assert_eq!(true, Passport::is_valid_hair_color("#aaaaaa"));
        assert_eq!(true, Passport::is_valid_hair_color("#a11aaa"));

        assert_eq!(false, Passport::is_valid_hair_color("#11111"));
        assert_eq!(false, Passport::is_valid_hair_color("#gggggg"));
        assert_eq!(false, Passport::is_valid_hair_color("#a11aaaaa"));

        assert_eq!(false, Passport::is_valid_hair_color("#123abz"));
        assert_eq!(false, Passport::is_valid_hair_color("a97842"));
    }
    #[test]
    fn is_valid_eye_color() {
        assert_eq!(true, Passport::is_valid_eye_color("amb"));
        assert_eq!(true, Passport::is_valid_eye_color("blu"));
        assert_eq!(true, Passport::is_valid_eye_color("brn"));

        assert_eq!(false, Passport::is_valid_eye_color("amber"));
        assert_eq!(false, Passport::is_valid_eye_color("#1234"));
        assert_eq!(false, Passport::is_valid_eye_color("abcd"));

        assert_eq!(false, Passport::is_valid_eye_color("wat"));
    }

    #[test]
    fn is_valid_pid() {
        assert_eq!(true, Passport::is_valid_pid("000000001"));
        assert_eq!(true, Passport::is_valid_pid("111111111"));

        assert_eq!(false, Passport::is_valid_pid("0123456789"));
    }
}
