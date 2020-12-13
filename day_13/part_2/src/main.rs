use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));

    let inputs: Vec<(i64, i64)> = parse_input(Box::new(aoc_reader));
    find_departure_time(inputs);
}

fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}

fn find_departure_time(periods_remainders: Vec<(i64, i64)>) {
    let mut pr_iter = periods_remainders.iter();
    let (mut running_product, _): (i64, i64) = *pr_iter.next().unwrap();
    let mut running_sum: i64 = 0;

    for (period, remainder) in pr_iter {
        while running_sum % period != *remainder {
            running_sum += running_product;
        }
        running_product *= period;
        println!("{}", running_product);
    }

    println!("{}", running_sum);   
}

fn parse_input(mut lines: Box<dyn Iterator<Item = String>>) -> Vec<(i64, i64)> {
    let _: i64 = lines.next().and_then(|line| {
            Some(line.as_str().parse::<i64>().unwrap())
        }
    ).unwrap();

    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"[^,]+"
        ).unwrap();
    }
    RE.find_iter(&lines.next().unwrap())
        .enumerate()
        .filter(|(_idx, capture)| {
            capture.as_str() != "x"
        })
        .map( |(idx, capture)| {
            let period = capture.as_str().parse::<i64>().unwrap();
            (
                period,
                period - idx as i64   
            )
        }).collect()
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



}