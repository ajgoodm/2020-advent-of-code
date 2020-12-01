use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SPECIAL_SUM: i32 = 2020;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let (x, y) = find_pair(f);
    println!("Found matching pair: {}, {} with product {}", x, y, x*y);
}

fn find_pair(read_buffer: BufReader<File>) -> (i32, i32) {
    let mut seen_numbers: HashSet<i32> = HashSet::new();
    for line in read_buffer.lines() {
        let num = line.expect("Unable to read line").parse::<i32>().unwrap();
        let complement: i32 = SPECIAL_SUM - num;
        if seen_numbers.contains(&complement) {
            return (num, complement)
        }
        seen_numbers.insert(num);
    }
    panic!("did not find a match!")
}