use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{char, one_of},
    combinator::{eof, map_res},
    multi::fold_many0,
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};

// AMAZING !!! https://github.com/Geal/nom/blob/master/tests/arithmetic.rs
fn main() {
    let input = include_str!("input");

    let results: u64 = input.lines().map(|l| solve_pt1(l)).sum();
    println!("Part 1: {}", results);

    let results: u64 = input.lines().map(|l| solve_pt2(l)).sum();
    println!("Part 2: {}", results);
}

fn solve_pt1(input: &str) -> u64 {
    fn operation(input: &str) -> IResult<&str, u64> {
        let (i, init) = paren_or_number(input).unwrap();
        // https://github.com/Geal/nom/blob/master/tests/arithmetic.rs#L37-L51
        fold_many0(
            pair(
                delimited(char(' '), one_of("+*"), char(' ')),
                paren_or_number,
            ),
            init,
            |acc, (op, val)| match op {
                '*' => acc * val,
                '+' => acc + val,
                _ => unreachable!(),
            },
        )(i)
    }

    fn paren_or_number(input: &str) -> IResult<&str, u64> {
        // pull out parentheses or else take the number
        delimited(char('('), operation, char(')'))
            .or(|l| nom_int_parse(l))
            .parse(input)
    }
    let (_, out) = terminated(operation, eof)(input).unwrap();
    out
}

fn solve_pt2(input: &str) -> u64 {
    // same as part 1 but now we split operation into 2, chaining product then sum (so we do plus first, then mul)

    fn sum(input: &str) -> IResult<&str, u64> {
        let (i, init) = paren_or_number(input).unwrap();
        fold_many0(preceded(tag(" + "), paren_or_number), init, |acc, val| {
            acc + val
        })(i)
    }

    fn product(input: &str) -> IResult<&str, u64> {
        let (i, init) = sum(input).unwrap();
        fold_many0(preceded(tag(" * "), sum), init, |acc, val| acc * val)(i)
    }

    fn paren_or_number(input: &str) -> IResult<&str, u64> {
        // pull out parentheses or else take the number
        delimited(char('('), product, char(')'))
            .or(|l| nom_int_parse(l))
            .parse(input)
    }
    let (_, out) = terminated(product, eof)(input).unwrap();
    out
}

fn is_decimal_digit(c: char) -> bool {
    c.is_digit(10)
}

fn nom_int_parse(input: &str) -> IResult<&str, u64> {
    map_res(take_while1(is_decimal_digit), str::parse)(input)
}

#[cfg(test)]
mod day18 {

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn examples_pt1() {
        let test = |input: &str, result: u64| assert_eq!(solve_pt1(input), result);

        test("1 + (2 * 3) + (4 * (5 + 6))", 51);
        test("2 * 3 + (4 * 5)", 26);
        test("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437);
        test("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240);
        test("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632);
    }

    #[test]
    fn examples_pt2() {
        let test = |input: &str, result: u64| assert_eq!(solve_pt2(input), result);

        test("2 * 3 + (4 * 5)", 46);
        test("1 + (2 * 3) + (4 * (5 + 6))", 51);
        test("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445);
        test("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060);
        test("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340);
    }
}
