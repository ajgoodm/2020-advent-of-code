use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

use regex::Regex;

fn main() {
    let required_keys: HashSet<String> = HashSet::from_iter(
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().map(
                |slice| slice.to_string())
    );

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let n_valid_passwords = count_valid_passports(f, &required_keys);
    println!("found {} valid passports", n_valid_passwords);
}


fn count_valid_passports(read_buffer: BufReader<File>, required_keys: &HashSet<String>) -> i32 {
    let mut n_valid_passports: i32 = 0;
    let passports = parse_passports(read_buffer);
    for passport in passports {
        if required_keys.is_subset(&passport.keys) {
            n_valid_passports = n_valid_passports + 1;
        }
    }
    n_valid_passports
}

struct PassportKeys {
    keys: HashSet<String>
}

fn parse_passports<'a>(read_buffer: BufReader<File>) -> Vec<PassportKeys> {
    let mut passports: Vec<PassportKeys> = Vec::new();
    let mut found_keys: HashSet<String> = HashSet::new();

    for line in read_buffer.lines() {
        let line = line.expect("could not read line!");
        if line != "" {
            found_keys = set_union(found_keys, parse_keys(&line)).clone(); 

        } else {
            passports.push(
                PassportKeys{
                    keys: found_keys
                }
            );
            found_keys = HashSet::new();
        }
    }
    if &found_keys.len() > &0 {
        passports.push(
            PassportKeys{
                keys: found_keys
            }
        );
    }
    passports
}

fn parse_keys(text: &str) -> HashSet<String> {
    let mut keys: HashSet<String> = HashSet::new();
    let re: Regex = Regex::new(
        r"([^ ]*):"
    ).unwrap();
    let captures: Vec<&str> = re.find_iter(text).map(|mat| mat.as_str()).collect();
    for cap in captures {
        keys.insert(cap[..cap.len() - 1].to_string());
    }
    keys
}

fn set_union(set_1: HashSet<String>, set_2: HashSet<String>) -> HashSet<String> {
    let mut new_set: HashSet<String> = HashSet::new();
    for item in set_1 {
        new_set.insert(item);
    }
    for item in set_2 {
        new_set.insert(item);
    }
    new_set
}