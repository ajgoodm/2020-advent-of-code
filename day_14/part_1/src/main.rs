use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use lazy_static::lazy_static;
use regex::Regex;

const N_BITS: usize = 36;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));

    calculate_sum(Box::new(aoc_reader));
}


fn calculate_sum(lines: Box<dyn Iterator<Item = String>>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: String = "".to_string() ;

    for line in lines {
        match &line[0..3] {
            "mas" => mask = parse_mask(&line),
            "mem" => {
                let (address, val) = parse_line(&line);
                *memory.entry(address).or_insert(apply_mask(val, &mask)) = apply_mask(val, &mask);
            },
            _ => panic!("Invalid line!")
        }
    }

    let sum: u64 = memory.values().sum();

    println!("final sum: {}", sum);
}


fn to_be_bits(val: &u64) -> [bool; N_BITS] {
    let mut mut_val = *val;

    let mut bits: [bool; N_BITS] = [false; N_BITS];
    for bit_idx in (0..N_BITS).rev() {
        if mut_val >= u64::pow(2, bit_idx as u32) {
            bits[N_BITS - 1 - bit_idx as usize] = true;
            mut_val -= u64::pow(2, bit_idx as u32);
        }
    }
    bits
}


fn from_be_bits(bits: &[bool; N_BITS]) -> u64 {
    bits.iter().rev().enumerate().map(
        |(idx, bit)| {
            match bit {
                true => u64::pow(2, idx as u32),
                false => 0
            }
        }
    ).sum()
}


fn parse_mask(line: &str) -> String {
    lazy_static! {
        static ref MASK_RE: Regex = Regex::new(
            r"^mask = ([10X]{36})$"
        ).unwrap();
    }
    let captures = MASK_RE.captures(line).unwrap();
    captures.get(1).unwrap().as_str().to_string()
}


fn parse_line(line: &str) -> (u64, u64) {
    lazy_static! {
        static ref VALS_RE: Regex = Regex::new(
            r"^mem\[([0-9]+)\] = ([0-9]+)$"
        ).unwrap();
    }
    let captures = VALS_RE.captures(line).unwrap();
    (
        captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
        captures.get(2).unwrap().as_str().parse::<u64>().unwrap()
    )
}


fn apply_mask(val: u64, mask: &String) -> u64 {
    let mut bits = to_be_bits(&val);
    for (idx, c) in mask.chars().enumerate() {
        match c {
            '1' => (bits[idx] = true),
            '0' => (bits[idx] = false),
            'X' => (),
            _ => panic!("not a valid character mask {}", c)
        }
    }
    from_be_bits(&bits)
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bits() {
        assert_eq!(to_be_bits(&0), [false; N_BITS]);
        assert_eq!(from_be_bits(&[false; N_BITS]), 0);

        let mut bits = [false; N_BITS];
        bits[0] = true;
        assert_eq!(to_be_bits(&u64::pow(2, N_BITS as u32 - 1)), bits);
        assert_eq!(from_be_bits(&bits), u64::pow(2, N_BITS as u32 - 1));

        bits[0] = false;
        bits[N_BITS - 1] = true;
        assert_eq!(to_be_bits(&1), bits);
        assert_eq!(from_be_bits(&bits), 1);

        bits[N_BITS - 2] = true;
        assert_eq!(to_be_bits(&3), bits);
        assert_eq!(from_be_bits(&bits), 3);
    }


    #[test]
    fn test_parse_mask() {
        assert_eq!(parse_mask("mask = 100110001110110011001X101110X1XX10X1"), "100110001110110011001X101110X1XX10X1");
    }


    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("mem[62998] = 9708340"), (62998, 9708340));
    }

    #[test]
    fn test_apply_mas() {
        assert_eq!(apply_mask(11, &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()), 73);
        assert_eq!(apply_mask(101, &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()), 101);
        assert_eq!(apply_mask(0, &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()), 64);
    }

}