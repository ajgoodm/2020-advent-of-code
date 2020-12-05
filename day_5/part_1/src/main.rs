use std::collections::{HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut seen_ids: HashSet<i32> = HashSet::new();
    let mut min_id: Option<i32> = None;
    let mut max_id: Option<i32> = None;


    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let max_id: i32 = f.lines().map(
        |line| {
            let id: i32 = parse_seat_id(&line.unwrap());
            seen_ids.insert(id);
            min_id = match min_id {
                None => Some(id),
                Some(min_id) => {
                    if id < min_id {
                        Some(id)
                    } else {
                        Some(min_id)
                    }
                }
            };
            id
        }
    ).max().unwrap();

    let all_seats: HashSet<i32> = HashSet::from_iter(
        (min_id.unwrap()..max_id + 1)
    );
    let missing_seats = all_seats.difference(&seen_ids);
    for seat in missing_seats {
        println!("missing seat {}", seat);
    }

    println!("The highest id is {}", max_id);
}


fn parse_binary(info: &str, zero_char: char, one_char: char) -> i32 {
    info.chars().rev().enumerate().map(
        | (idx, c) | {
            if c == zero_char {
                return 0
            } else if c == one_char {
                return i32::pow(2, idx as u32)
            }
            panic!("something went wrong!");
        }
    ).sum()
}


fn parse_seat_id(seat: &str) -> i32 {
    let re: Regex = Regex::new(
        r"^([FB]{7})([LR]{3})$"
    ).unwrap();
    let (row_info, col_info) = re.captures(seat).and_then(
        |cap| {Some((
            cap.get(1).unwrap().as_str(),
            cap.get(2).unwrap().as_str()
        ))}
    ).unwrap();

    parse_binary(row_info, 'F', 'B') * 8
    + parse_binary(col_info, 'L', 'R')
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_binary_info() {
        assert_eq!(parse_binary("BFFFBBF", 'F', 'B'), 70);
        assert_eq!(parse_binary("FFFBBBF", 'F', 'B'), 14);
        assert_eq!(parse_binary("BBFFBBF", 'F', 'B'), 102);
        assert_eq!(parse_binary("RRR", 'L', 'R'), 7);
        assert_eq!(parse_binary("RLL", 'L', 'R'), 4);
    }

    #[test]
    fn test_parse_seat_id() {
        assert_eq!(parse_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(parse_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(parse_seat_id("BBFFBBFRLL"), 820);
    }
}