use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));

    let (start_time, periods) = parse_input(Box::new(aoc_reader));
    find_departure_time(start_time, periods);
}

fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}

fn find_departure_time(start_time: i64, periods: Vec<i64>) {
    println!("start time: {}", start_time);
    for p in &periods {
        println!("{}", p);
    }
    /*
        buses arrive at integer multiples
        of the periods. For any given perirod, 
        the next bus will arrive at

        p - start % p
    */
    let wait_times: Vec<(i64, i64)> = periods.iter()
        .map( |period| {
            (*period, period - (start_time % period))
        }).collect();

    // Rust does not have a min_by_key except
    // in it's nightly experimental release.
    // Blech.
    let mut min_wait_time: Option<i64> = None;
    let mut min_period: Option<i64> = None;
    for (period, wait_time) in wait_times {
        if min_wait_time == None {
            min_wait_time = Some(wait_time);
            min_period = Some(period);
        } else if wait_time < min_wait_time.unwrap() {
            min_wait_time = Some(wait_time);
            min_period = Some(period);
        }
    }

    println!(
        "period: {}, wait: {}, product: {}",
        min_period.unwrap(),
        min_wait_time.unwrap(),
        min_period.unwrap() * min_wait_time.unwrap()
    )
}

fn parse_input(mut lines: Box<dyn Iterator<Item = String>>) -> (i64, Vec<i64>) {
    let start_time: i64 = lines.next().and_then(|line| {
            Some(line.as_str().parse::<i64>().unwrap())
        }
    ).unwrap();

    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"[0-9]+"
        ).unwrap();
    }
    let periods: Vec<i64> = RE.find_iter(&lines.next().unwrap())
        .map( |val|
            val.as_str().parse::<i64>().unwrap()
        ).collect();

    (start_time, periods)
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