use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Sum;
use std::ops::Add;

use lazy_static::lazy_static;
use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));

    println!("Number of black tiles: {}", count_flips(Box::new(aoc_reader)));
}


fn count_flips(reader: Box<dyn Iterator< Item = String >>) -> usize {
    let mut counts: HashMap<CoordHexBasis, usize> = HashMap::new();
    for line in reader {
        let dest: CoordHexBasis = parse_directions(&line).iter().map(|d| d.as_coord()).sum::<CoordHexBasis>();
        *counts.entry(dest).or_insert(0usize) += 1;
    }
    
    counts.values()
    .filter(|v| !is_even(**v))
    .count()
}


fn parse_directions(line: &str) -> Vec<Direction> {
    lazy_static!{
        static ref NEXT: Regex = Regex::new("^(e|se|sw|w|nw|ne)(.*)$").unwrap();
    }
    let mut input = line.clone();
    let mut result: Vec<Direction> = vec![];
    loop {
        let captures = NEXT.captures(input);
        match captures {
            Some(captures) => {
                match captures.get(1).unwrap().as_str() {
                    "e" => result.push(Direction::e),
                    "se" => result.push(Direction::se),
                    "sw" => result.push(Direction::sw),
                    "w" => result.push(Direction::w),
                    "nw" => result.push(Direction::nw),
                    "ne" => result.push(Direction::ne),
                    _ => panic!("received invalid input!")
                };
                input = captures.get(2).unwrap().as_str();
            },
            None => break
        }
    }
    result
}


fn is_even(n: usize) -> bool {
    if n % 2 == 0 {true} else {false}
}


#[derive(Debug, PartialEq)]
enum Direction {
    e,
    se,
    sw,
    w,
    nw,
    ne
}


impl Direction {
    fn as_coord(&self) -> CoordHexBasis {
        match self {
            Direction::e => CoordHexBasis { u: 1, v: 0 },
            Direction::se => CoordHexBasis { u: 1, v: -1 },
            Direction::sw => CoordHexBasis { u: 0, v: -1},
            Direction::w => CoordHexBasis { u: -1, v: 0 },
            Direction::nw => CoordHexBasis { u: -1, v: 1 },
            Direction::ne => CoordHexBasis { u: 0, v: 1 }
        }
    }
}


#[derive(Debug, PartialEq, Eq, Hash)]
struct CoordHexBasis {
    u: isize, // +e direction
    v: isize  // +ne direction
}


impl Add for CoordHexBasis {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            u: self.u + other.u,
            v: self.v + other.v,
        }
    }
}


impl<'a> Sum<Self> for CoordHexBasis {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self { u: 0, v: 0 }, |a, b| a + b)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_directions() {
        assert_eq!(parse_directions("esenee"), vec![Direction::e, Direction::se, Direction::ne, Direction::e]);
        assert_eq!(parse_directions(""), vec![]);
        assert_eq!(parse_directions("esew"), vec![Direction::e, Direction::se, Direction::w]);
    }

    fn test_as_coord() {
        assert_eq!(Direction::e.as_coord(), CoordHexBasis { u: 1, v: 0 });
        assert_eq!(Direction::w.as_coord(), CoordHexBasis { u: -1, v: 0 });
        assert_eq!(Direction::ne.as_coord(), CoordHexBasis { u: 0, v: 1 });
        assert_eq!(Direction::sw.as_coord(), CoordHexBasis { u: 0, v: -1 });
        assert_eq!(Direction::se.as_coord(), CoordHexBasis { u: 1, v: -1 });
        assert_eq!(Direction::nw.as_coord(), CoordHexBasis { u: -1, v: 1 });
    }

    fn test_sum() {
        assert_eq!(
            parse_directions("esew").iter().map(|d| d.as_coord()).sum::<CoordHexBasis>(),
            CoordHexBasis { u: 1, v: -1 }
        );
        assert_eq!(
            parse_directions("nwwswee").iter().map(|d| d.as_coord()).sum::<CoordHexBasis>(),
            CoordHexBasis { u: 0, v: 0 }
        )
    } 
}

fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}

struct AocBufReader {
    iter: Lines<BufReader<File>>,
}

impl AocBufReader {
    fn from_file(file_handle: File) -> AocBufReader {
        AocBufReader {
            iter: BufReader::new(file_handle).lines()
        }
    }
}

impl Iterator for AocBufReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(result) => match result {
                Ok(line) => Some(line),
                Err(error) => panic!(error)
            },
            None => None
        }
    }
}