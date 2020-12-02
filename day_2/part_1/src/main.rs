use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let n_valid_passwords = count_valid_passwords(f);
    
    println!("found {} valid passwords", n_valid_passwords);
}


fn count_valid_passwords(read_buffer: BufReader<File>) -> i32 {
    let mut n_matches: i32 = 0;
    for line in read_buffer.lines() {
        let line = line.expect("Could not read line!");
        let (password_rule, input) = extract_rule_password(&line);
        let n_occurences: i32 = input.matches(&password_rule.letter).count() as i32;
        if password_rule.min_occurences <= n_occurences && n_occurences <= password_rule.max_occurences {
            n_matches = n_matches + 1;
        }
    }
    n_matches
}


struct PasswordRule {
    letter: String,
    min_occurences: i32,
    max_occurences: i32,
}


fn extract_rule_password(text: &str) -> (PasswordRule, &str) {
    let re: Regex = Regex::new(
        r"([0-9]*)\-([0-9]*) ([a-z]): ([^ ]*)"
    ).unwrap();
    let captures = re.captures(text).unwrap();

    let (min_occurences, max_occurences): (i32, i32) = (
        captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        captures.get(2).unwrap().as_str().parse::<i32>().unwrap()
    );
    let (letter, input): (&str, &str) = (captures.get(3).unwrap().as_str(), captures.get(4).unwrap().as_str());

    (
        PasswordRule {
            letter: letter.to_string(),
            min_occurences: min_occurences,
            max_occurences: max_occurences
        },
        input
    )
}