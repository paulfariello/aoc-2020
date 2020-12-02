#[macro_use]
extern crate log;

use std::fs::File;
use std::io::{Read, BufRead, BufReader};

fn load_values(input: &mut dyn Read) -> Vec<u64> {
    let reader = BufReader::new(input);
    let mut values: Vec<u64> = Vec::new();
    for line in reader.lines() {
        let n = line.unwrap().parse::<u64>().unwrap();
        values.push(n);
    }

    values
}

fn find_matching_pair(values: &mut Vec<u64>, target: u64) -> Option<(u64, u64)> {
    values.sort_unstable();

    let mut start = 0;
    let mut end = values.len() - 1;
    loop {
        let sum = values[start] + values[end];
        if sum == target {
            return Some((values[start], values[end]));
        } else if sum < target {
            start += 1;
        } else if sum > target {
            end -= 1;
        } else {
            unreachable!();
        }

        if start > end {
            break;
        }
    }

    None
}

fn find_matching_3_tuple(values: &mut Vec<u64>) -> Option<(u64, u64, u64)> {
    values.sort_unstable();

    for i in values.clone().iter().cloned() {
        let diff = 2020 - i;
        if let Some((a, b)) = find_matching_pair(values, diff) {
            return Some((a, b, i));
        }
    }

    None
}

fn solve_1(input_file: &str) -> Result<u64, &str> {
    info!("AoC 2020-01 - part 1");
    let mut input = File::open(input_file).unwrap();
    let mut values = load_values(&mut input);

    match find_matching_pair(&mut values, 2020) {
        Some((a, b)) => Ok(a * b),
        None => Err("No matching pair"),
    }
}

fn solve_2(input_file: &str) -> Result<u64, &str> {
    info!("AoC 2020-01 - part 2");
    let mut input = File::open(input_file).unwrap();
    let mut values = load_values(&mut input);

    match find_matching_3_tuple(&mut values) {
        Some((a, b, c)) => Ok(a * b * c),
        None => Err("No matching 3-tuple"),
    }
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
    fn test_1() {
        let result = solve_1("tests.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 514579);
    }

    #[test]
    fn test_2() {
        let result = solve_2("tests.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 241861950);
    }
}
