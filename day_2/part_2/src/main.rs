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
        if line_is_valid(&line.expect("Could not read line!")) {
            n_matches = n_matches + 1;
        } 
    }
    n_matches
}


fn line_is_valid(line: &str) -> bool {
    let (password_rule, input) = extract_rule_password(&line);
    // This is only OK, because we're dealing with standard ASCII data
    (&input[password_rule.first_idx..password_rule.first_idx + 1] == password_rule.letter)
     ^ (&input[password_rule.second_idx..password_rule.second_idx + 1] == password_rule.letter)
}


struct PasswordRule {
    letter: String,
    first_idx: usize,
    second_idx: usize,
}


fn extract_rule_password(text: &str) -> (PasswordRule, &str) {
    let re: Regex = Regex::new(
        r"([0-9]*)\-([0-9]*) ([a-z]): ([^ ]*)"
    ).unwrap();
    let captures = re.captures(text).unwrap();

    let (first_idx, second_idx): (usize, usize) = (
        captures.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1,
        captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1
    );
    let (letter, input): (&str, &str) = (captures.get(3).unwrap().as_str(), captures.get(4).unwrap().as_str());

    (
        PasswordRule {
            letter: letter.to_string(),
            first_idx: first_idx,
            second_idx: second_idx
        },
        input
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_is_valid() {
        assert_eq!(line_is_valid("1-3 a: abcde"), true);
        assert_eq!(line_is_valid("1-3 b: cdefg"), false);
        assert_eq!(line_is_valid("2-9 c: ccccccccc"), false);
    }
}