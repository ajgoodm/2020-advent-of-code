use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SPECIAL_SUM: i32 = 2020;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let (x, y, z) = find_triplet(f);
    println!("Found matching triplet: {}, {}, {} with product {}", x, y, z, x*y*z);
}

fn find_triplet(read_buffer: BufReader<File>) -> (i32, i32, i32) {
    let mut seen_numbers: HashMap<i32, HashSet<i32>> = HashMap::new();
    // learning rust, and I could not for the life of me
    // figure out how to iterate over keys and mutate the
    // values (HashSet's)
    let mut keys: HashSet<i32> = HashSet::new();
    
    for line in read_buffer.lines() {
        let num = line.expect("Unable to read line").parse::<i32>().unwrap();

        for num_1 in &keys {
            let complement: i32 = SPECIAL_SUM - *num_1 - num;
            let hash_map = seen_numbers.get_mut(&num_1).unwrap();
            if hash_map.contains(&num) {
                return (*num_1, num, SPECIAL_SUM - num - *num_1)
            }
            hash_map.insert(complement);
        }

        seen_numbers.insert(num, HashSet::new());
        keys.insert(num);
    }
    panic!("did not find a match!")
}