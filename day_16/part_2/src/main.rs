use std::collections::{HashMap, HashSet};
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
    let mut solver: Solver = Solver::from_range_map(ranges_map);
    for ticket in nearby_tickets {
        solver.add_ticket(ticket);
    }
    solver.solve();

    
    let product: i64 = solver.class_idx_map.iter().filter(
        |(class, _idx)| {class.len() >= 9 && &class[..9] == "departure"}
    ).map(
        |(class, _idx)| {
            your_ticket.vals[
                Solver::get_one_or_none_idx(
                    solver.get_row(*solver.class_idx_map.get(class).unwrap())
                ).unwrap()
            ]
        }
    ).product();
    println!("{}", product);
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


struct Solver {
    range_map: HashMap<String, Ranges>,
    class_idx_map: HashMap<String, usize>,
    grid: Vec<Vec<bool>>
}


impl Solver {
    fn from_range_map(range_map: HashMap<String, Ranges>) -> Solver {
        let mut class_idx_map: HashMap<String, usize> = HashMap::new();
        for (idx, class) in range_map.keys().enumerate() {
            class_idx_map.insert(class.to_string(), idx as usize);
        }
        let grid = vec![
            vec![true; class_idx_map.keys().len()]; class_idx_map.keys().len()
        ];
        Solver {
            range_map: range_map,
            class_idx_map: class_idx_map,
            // the row corresponds to the key
            // the column corresponds to the slot
            grid: grid
        }
    }

    fn n_rows_cols(&self) -> usize {
        self.grid.len()
    }

    fn is_solved(&self) -> bool {
        for row in &self.grid {
            if row.iter().filter(|x| **x).count() != 1 {
                return false
            }
        }
        true
    }

    fn get_row(&self, row_idx: usize) -> Vec<bool> {
        self.grid[row_idx].iter().map(|x| *x).collect()
    }
    
    fn get_col(&self, col_idx: usize) -> Vec<bool> {
        self.grid.iter().map(|row| row[col_idx]).collect()
    }

    fn add_ticket(&mut self, ticket: Ticket) {
        if !ticket.is_valid(&self.range_map) { return }
        for (slot, val) in ticket.vals.iter().enumerate() {
            for (class, ranges) in &self.range_map {
                if !ranges.any_range_contains(&val) {
                    self.grid[*self.class_idx_map.get(class).unwrap()][slot] = false;
                }
            }
        }
    }

    fn get_one_or_none_idx(vec: Vec<bool>) -> Option<usize> {
        let results: Vec<(usize, bool)> = vec.iter().enumerate()
            .filter(|(idx, x)| **x)
            .map(|(idx, x)| (idx, *x))
            .collect();
        match results.len() {
            1 => {
                let (idx, _val) = results[0];
                Some(idx)
            } 
            _ => None
        }
    }

    fn solve(&mut self) {
        while !self.is_solved() {
            for candidate_idx in 0..self.n_rows_cols() {
                match Solver::get_one_or_none_idx(self.get_row(candidate_idx)) {
                    Some(idx) => {
                        for ii in (0..self.n_rows_cols()).filter(|ii| *ii != candidate_idx) {
                            self.grid[ii][idx] = false
                        }
                    },
                    None => ()
                }
            }
            for candidate_idx in 0..self.n_rows_cols() {
                match Solver::get_one_or_none_idx(self.get_col(candidate_idx)) {
                    Some(idx) => {
                        for ii in (0..self.n_rows_cols()).filter(|ii| *ii != candidate_idx) {
                            self.grid[idx][ii] = false
                        }
                    },
                    None => ()
                }
            }
        }
    }
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

impl Ticket {
    fn is_valid(&self, range_map: &HashMap<String, Ranges>) -> bool {
        for val in self.vals.iter() {
            if !any_class_range_contains(&val, range_map) { return false }
        }
        true
    }
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

    #[test]
    fn test_get_one_or_none_idx() {
        assert_eq!(Solver::get_one_or_none_idx(vec![true, false, false]), Some(0));
        assert_eq!(Solver::get_one_or_none_idx(vec![true, true, false]), None);
        assert_eq!(Solver::get_one_or_none_idx(vec![false, false, false]), None);
    }
}