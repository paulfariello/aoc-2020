#[macro_use]
extern crate log;

use std::fs::File;
use std::io::{BufRead, BufReader};
use nom::IResult;
use nom::sequence::tuple;
use nom::combinator::{all_consuming};
use nom::character::complete::{digit1, char, alpha1, space1, one_of};

#[derive(PartialEq, Debug)]
struct Entry {
    count: (usize, usize),
    c: char,
    password: String,
}

impl Entry {
    pub fn check_policy_1(&self) -> bool {
        let count = self.password.chars().filter(|a| *a == self.c).count();
        count >= self.count.0 && count <= self.count.1
    }

    pub fn check_policy_2(&self) -> bool {
        let bytes = self.password.as_bytes();
        let byte = self.c as u8;

        let a = bytes[self.count.0 - 1] == byte;
        let b = bytes[self.count.1 - 1] == byte;

        (a && !b) || (b && !a)
    }
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    match all_consuming(tuple((digit1, char('-'), digit1, space1, one_of("abcdefghijklmnopqrstuvwxyz"),
                               char(':'), space1, alpha1)))(input) {
        Ok((_, (min, _, max, _, c, _, _, password))) => Ok(("", Entry {
            count: (str::parse(min).unwrap(), str::parse(max).unwrap()),
            c,
            password: password.to_string()
        })),
        Err(e) => Err(e),
    }
}

fn solve_1(input_file: &str) -> Result<u64, &str> {
    info!("AoC 2020-02 - part 1");
    let input = File::open(input_file).unwrap();
    let reader = BufReader::new(input);
    let mut count = 0;
    for line in reader.lines() {
        let (_, entry) = parse_entry(&line.unwrap()).unwrap();
        if entry.check_policy_1() {
            count += 1;
        }
    }

    Ok(count)
}

fn solve_2(input_file: &str) -> Result<u64, &str> {
    info!("AoC 2020-02 - part 2");
    let input = File::open(input_file).unwrap();
    let reader = BufReader::new(input);
    let mut count = 0;
    for line in reader.lines() {
        let (_, entry) = parse_entry(&line.unwrap()).unwrap();
        if entry.check_policy_2() {
            count += 1;
        }
    }

    Ok(count)
}

fn main() {
    let logger = flexi_logger::Logger::with_env_or_str("debug");
    if let Err(e) = logger.start() {
        panic!("Cannot start logger: {}", e);
    }

    if let Ok(result) = solve_1("input.txt") {
        info!("Part 1: {}", result);
    }

    if let Ok(result) = solve_2("input.txt") {
        info!("Part 2: {}", result);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entry() {
        let entry = Entry {
            count: (1, 12),
            c: 'a',
            password: "abc".to_string(),
        };

        assert_eq!(parse_entry("1-12 a: abc"), Ok(("", entry)));
    }

    #[test]
    fn test_1() {
        let result = solve_1("tests.txt");
        assert_eq!(result, Ok(2));
    }

    #[test]
    fn test_2() {
        let result = solve_2("tests.txt");
        assert_eq!(result, Ok(1));
    }
}
