use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const PREAMBLE_SIZE: usize = 25;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));
    let imposter: i64 = find_imposter(Box::new(aoc_reader));
    println!("The imposter is {}", imposter);

    let aoc_reader = AocBufReader::from_file(open_file(filename));
    let special_sum: i64 = find_contiguous_block(Box::new(aoc_reader), &imposter);
    println!("The special sum is {}", special_sum);

}

fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}

fn find_contiguous_block(lines: Box<dyn Iterator<Item = String>>, imposter: &i64) -> i64 {
    let mut vals: Vec<i64> = vec![];
    for line in lines {
        vals.push(line.parse::<i64>().unwrap());
    }

    let (start, end) = find_slice_bounds(&vals, imposter);

    *vals[start..end].iter().min().unwrap()
    + *vals[start..end].iter().max().unwrap()
}

fn find_slice_bounds(vals: &Vec<i64>, imposter: &i64) -> (usize, usize) {
    for idx_1 in 0..(vals.len() - 1) {
        let mut sum = vals[idx_1];
        for idx_2 in (idx_1 + 1)..vals.len() {
            sum += vals[idx_2];
            if sum > *imposter {
                break;
            } else if sum == *imposter {
                return (idx_1, idx_2 + 1)
            }
        }
    }
    panic!("We didn't find a block!");
}

fn find_imposter(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut buffer: [i64;PREAMBLE_SIZE] = [-1; PREAMBLE_SIZE];
    for idx in 0..PREAMBLE_SIZE{
        buffer[idx as usize] = lines.next().unwrap().parse::<i64>().unwrap()
    }
    
    // the buffer is acting as a hacky ring cache!
    let mut candidate: i64 = lines.next().unwrap().parse::<i64>().unwrap();
    let mut first_out_ptr: usize = 0;
    while is_match_found(&mut buffer, &candidate) {
        buffer[first_out_ptr] = candidate;
        first_out_ptr = (first_out_ptr + 1) % PREAMBLE_SIZE;
        candidate = lines.next().unwrap().parse::<i64>().unwrap()
    }

    candidate
}

fn is_match_found(buffer: &mut [i64], candidate: &i64) -> bool {
    for idx_1 in 0..(PREAMBLE_SIZE - 1) {
        for idx_2 in (idx_1 + 1)..PREAMBLE_SIZE {
            if buffer[idx_1] + buffer[idx_2] == *candidate {
                return true
            }
        } 
    }
    false
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