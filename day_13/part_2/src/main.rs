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

    // There some funky stuff happening here. We iteratively find
    // a time that works for each remainder. _A_ time t_i which
    // equals r_i mod p_i for each remainder. When we move to the
    // next bus, we want to find a time t_i+1 that satisfies all
    // previous buses as well. We need to take the previous time
    // and add the least common multiple (LCM) of the previous buses
    // periods, because this will be the smallest number that is
    // 0 mod all the previous ps. Our buses all have priume periods,
    // so the LCM of any collection of buses is their product.
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
            // The lore says the index in this list is how many
            // minutes after our special time t_0, the bus with
            // this period will need to arrive. That means that
            // the special time t_0 occurs idx  units _before_
            // a time which is 0 mod p (the period of the bus)
            let mut idx = period - idx as i64;

            // we only care about the "remainder" modulo the
            // period (the bus ID), so we can +/- period with
            // impunity.
            while idx < 0{idx += period}
            (period, idx)
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
