use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use lazy_static::lazy_static;
use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));
    let (ranges_map, your_ticket, nearby_tickets) = parse_input(Box::new(aoc_reader));

    let mut sum: i64 = 0;
    for ticket in nearby_tickets {
        for val in ticket.vals {
            if !any_class_range_contains(&val, &ranges_map) { sum += val }
        }
    }
    println!("final sum: {}", sum);

}


fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}


fn parse_input(mut reader: Box<dyn Iterator<Item = String>>) -> (HashMap<String, Ranges>, Ticket, Vec<Ticket>) {
    let mut ranges_map: HashMap<String, Ranges> = HashMap::new();
    
    loop {
        let line = reader.next().unwrap();
        if line == "" {break;}

        let (class, ranges) = parse_class_range(&line);
        ranges_map.insert(class, ranges);
    }
    
    let line = reader.next().unwrap();
    assert_eq!(line, "your ticket:");
    let your_ticket: Ticket = parse_ticket(&reader.next().unwrap());

    reader.next();
    reader.next();
    let mut tickets: Vec<Ticket> = vec![];
    for line in reader {
        tickets.push(parse_ticket(&line));
    }

    (ranges_map, your_ticket, tickets)
}


fn parse_class_range(line: &str) -> (String, Ranges) {
    lazy_static! {
        static ref RANGE_RE: Regex = Regex::new(
            r"^([^:]+): ([0-9]+)\-([0-9]+) or ([0-9]+)\-([0-9]+)$"
        ).unwrap();
    }
    let captures = RANGE_RE.captures(line).unwrap();
    (
        captures.get(1).unwrap().as_str().to_string(),
        Ranges {
            ranges: vec![
                Range {
                    min: captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                    max: captures.get(3).unwrap().as_str().parse::<i64>().unwrap()
                },
                Range {
                    min: captures.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                    max: captures.get(5).unwrap().as_str().parse::<i64>().unwrap()
                }
            ]
        }
    )
}


fn parse_ticket(line: &str) -> Ticket {
    let vals: Vec<i64> = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
    Ticket {
        vals: vals
    }
}


fn any_class_range_contains(val: &i64, range_map: &HashMap<String, Ranges>) -> bool {
    for ranges in range_map.values() {
        if ranges.any_range_contains(&val) {return true}
    }
    return false
}


struct Ranges {
    ranges: Vec<Range>
}

impl Ranges {
    fn any_range_contains(&self, val: &i64) -> bool {
        for range in self.ranges.iter() {
            if range.contains(&val) { return true; }
        }
        false
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Range {
    min: i64,
    max: i64
}

impl Range {
    fn contains(&self, val: &i64) -> bool {
        (*val >= self.min) && (*val <= self.max)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Ticket {
    vals: Vec<i64>
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_class_range() {
        let (class, ranges) = parse_class_range("class: 1-3 or 5-7");
        assert_eq!(class, "class".to_string());
        assert_eq!(ranges.ranges[0], Range {min: 1, max: 3});
        assert_eq!(ranges.ranges[1], Range {min: 5, max: 7});

        let (class, ranges) = parse_class_range("row: 6-11 or 33-44");
    }

    
    #[test]
    fn test_parse_ticket() {
        assert_eq!(parse_ticket("7,1,14"), Ticket { vals: vec![7, 1, 14] });
    }
}