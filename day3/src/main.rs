use std::{fs, path::PathBuf};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, digit1},
    combinator::opt,
    multi::many_till,
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let result = part_one("./day3/input.txt".into());
    println!("Part one: {result}");

    let result = part_two("./day3/input.txt".into());
    println!("Part two: {result}");
}

fn part_one(path: PathBuf) -> usize {
    let text = fs::read_to_string(path).unwrap();
    let mut parser = Parser::new(&text);
    parser.parse();
    parser.total
}
fn part_two(path: PathBuf) -> usize {
    let text = fs::read_to_string(path).unwrap();
    let mut parser = Parser::new(&text);
    parser.parse();
    parser.total
}

struct Parser<'a> {
    rest: &'a str,
    enabled: bool,
    total: usize,
}

impl<'a> Parser<'a> {
    fn new(rest: &'a str) -> Self {
        Parser {
            rest,
            enabled: true,
            total: 0,
        }
    }

    fn parse(&mut self) -> usize {
        let mut curr_len = self.rest.len();
        loop {
            let _ = self.parse_chunk();
            if curr_len == self.rest.len() {
                break;
            } else {
                curr_len = self.rest.len()
            }
        }
        self.total
    }

    fn parse_chunk(&mut self) -> IResult<&str, Option<(&str, &str)>> {
        let (input, token) = many_till(
            take(1usize),
            alt((Parser::parse_mul, Parser::parse_do, Parser::parse_dont)),
        )(self.rest)?;
        match token.1 {
            "do()" => self.enabled = true,
            "don't()" => self.enabled = false,
            _ => {}
        };
        let (input, inner) = opt(delimited(char('('), Parser::parse_inner, char(')')))(input)?;
        self.rest = input;
        if let Some(inner) = inner {
            let (x, y): (usize, usize) = (inner.0.parse().unwrap(), inner.1.parse().unwrap());
            if self.enabled {
                self.total += x * y
            }
        }
        Ok((input, inner))
    }

    fn parse_mul(input: &str) -> IResult<&str, &str> {
        tag("mul")(input)
    }

    fn parse_do(input: &str) -> IResult<&str, &str> {
        tag("do()")(input)
    }
    fn parse_dont(input: &str) -> IResult<&str, &str> {
        tag("don't()")(input)
    }

    fn parse_inner(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(digit1, char(','), digit1)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part_1_test() {
        assert_eq!(161, part_one("./test.txt".into()))
    }

    #[test]
    fn day3_part_2_test() {
        assert_eq!(48, part_one("./test2.txt".into()))
    }
}
