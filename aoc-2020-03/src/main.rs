#[macro_use]
extern crate log;

use std::fs::File;
use std::io::{Read, BufRead, BufReader};

#[derive(Debug, Clone)]
enum Place {
    Free,
    Tree,
}

#[derive(Debug, Clone)]
struct Map {
    data: Vec<Vec<Place>>,
    width: Option<usize>,
    height: usize,
}

impl Map {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            width: None,
            height: 0,
        }
    }

    pub fn add_row(&mut self, line: String) {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '.' => Place::Free,
                '#' => Place::Tree,
                _ => unreachable!(),
            });
        }

        match self.width {
            None => self.width = Some(row.len()),
            Some(width) => assert_eq!(width, row.len()),
        }

        self.data.push(row);
        self.height += 1;
    }

    pub fn place(&self, x: usize, y: usize) -> Place {
        self.data[x][y % self.width.unwrap()].clone()
    }

    pub fn count_tree(&self, right: usize, down: usize) -> u64 {
        let (mut x, mut y) = (0, 0);
        let mut count = 0;

        while x < self.height {
            count += match self.place(x, y) {
                Place::Free => 0,
                Place::Tree => 1,
            };

            x += down;
            y += right;
        }

        count
    }
}

fn load_map(input: &mut dyn Read) -> Map {
    let reader = BufReader::new(input);
    let mut map = Map::new();

    for line in reader.lines() {
        map.add_row(line.unwrap());
    }

    map
}

fn solve_1(input_file: &str) -> Result<u64, &str> {
    info!("AoC 2020-01 - part 1");
    let mut input = File::open(input_file).unwrap();
    let map = load_map(&mut input);

    Ok(map.count_tree(3, 1))
}

fn solve_2(input_file: &str) -> Result<u64, &str> {
    info!("AoC 2020-01 - part 2");
    let mut input = File::open(input_file).unwrap();
    let map = load_map(&mut input);

    Ok(map.count_tree(1, 1) * map.count_tree(3, 1) * map.count_tree(5, 1) * map.count_tree(7, 1) * map.count_tree(1, 2))
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
        assert_eq!(result, Ok(7));
    }

    #[test]
    fn test_2() {
        let result = solve_2("tests.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 336);
    }
}
