use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use lazy_static::lazy_static;
use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));

    let mut sum: i64 = 0;
    for line in aoc_reader {
        sum += evaluate_expression(&line);
    }
    println!("{}", sum);


}


fn evaluate_expression(exp: &str) -> i64 {
    let flat_expression = replace_parentheses(exp.to_string());
    evaluate_flat_expression(&flat_expression)
}


fn evaluate_flat_expression(exp: &str) -> i64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(.*) ([\+*]{1}) ([0-9]+)$"
        ).unwrap();
    }
    lazy_static! {
        static ref FINAL: Regex = Regex::new(
            r"^([0-9]) ([\+*]{1}) ([0-9])$"
        ).unwrap();
    }
    lazy_static! {
        static ref SINGLE_NUMBER: Regex = Regex::new(
            r"^([0-9]+)$"
        ).unwrap();
    }
    
    match SINGLE_NUMBER.captures(exp) {
        Some(captures) => return captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
        None => ()
    }

    match FINAL.captures(exp) {
        Some(captures) => {
            let (arg1, operator, arg2): (i64, &str, i64) = (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str(),
                captures.get(3).unwrap().as_str().parse::<i64>().unwrap()
            );
            match operator {
                "*" => return arg1 * arg2,
                "+" => return arg1 + arg2,
                _ => panic!("something went wrong parsing final expr.")
            }
        }
        None => ()
    }


    let captures = RE.captures(exp).unwrap();
    let (remainder, operator, arg2): (&str, &str, i64) = (
        captures.get(1).unwrap().as_str(),
        captures.get(2).unwrap().as_str(),
        captures.get(3).unwrap().as_str().parse::<i64>().unwrap()
    );
    match operator {
        "*" => evaluate_flat_expression(remainder) * arg2,
        "+" =>  evaluate_flat_expression(remainder) + arg2,
        _ => panic!("something went wrong parsing intermediate expr!")
    }
}


fn replace_parentheses(exp: String) -> String {
    lazy_static! {
        static ref INNER_PAREN: Regex = Regex::new(
            r"^(.*)\(([^\(\)]*)\)(.*)$"
        ).unwrap();
    }
    match INNER_PAREN.captures(&exp) {
        Some(captures) => {
            let (mut left, middle, right): (String, String, String) = (
                captures.get(1).unwrap().as_str().to_string(),
                captures.get(2).unwrap().as_str().to_string(),
                captures.get(3).unwrap().as_str().to_string(),
            );
                left.push_str(&evaluate_flat_expression(&middle).to_string());
                left.push_str(&right);
                replace_parentheses(left)
        },
        None => exp
    }
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
    fn test_evaluate_flat_exp() {
        assert_eq!(evaluate_flat_expression("1 + 2 * 3 + 4 * 5 + 6"), 71);
    }

    #[test]
    fn test_replace_parentheses() {
        assert_eq!(replace_parentheses("1 + (2 * 3) + 4 * 5 + 6".to_string()), "1 + 6 + 4 * 5 + 6".to_string());
        assert_eq!(replace_parentheses("1 + (2 * 3 + 4) * 5 + 6".to_string()), "1 + 10 * 5 + 6".to_string());
        assert_eq!(replace_parentheses("1 + (2 * 3) + (4 * 5) + 6".to_string()), "1 + 6 + 20 + 6".to_string());
        assert_eq!(replace_parentheses("1 + ((2 * 3) + 4) * 5 + 6".to_string()), "1 + 10 * 5 + 6".to_string());
        assert_eq!(replace_parentheses("1 + (2 * 3) + (4 * (5 + 6))".to_string()), "1 + 6 + 44".to_string());
    }

    #[test]
    fn test_evaluate_exp() {
        assert_eq!(evaluate_expression("2 * 3 + (4 * 5)"), 26);
        assert_eq!(evaluate_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    }
}