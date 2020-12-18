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
        static ref SINGLE_NUMBER: Regex = Regex::new(
            r"^([0-9]+)$"
        ).unwrap();
    }
    lazy_static! {
        static ref ADDN_RE: Regex = Regex::new(
            r"^(.*\*.* )?([0-9]+) (\+{1}) ([0-9]+)(.*)$"
        ).unwrap();
    }
    lazy_static! {
        static ref MULT_RE: Regex = Regex::new(
            r"^([0-9]+) (\*{1}) ([0-9]+)(.*)$"
        ).unwrap();
    }

    
    match SINGLE_NUMBER.captures(exp) {
        Some(captures) => return captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
        None => ()
    }

    match ADDN_RE.captures(exp) {
        Some(captures) => {
            let (left, arg1, _operator, arg2, right): (Option<regex::Match>, i64, &str, i64, Option<regex::Match>) = (
                captures.get(1),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(3).unwrap().as_str(),
                captures.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(5)
            );
            let mut result: String = match left {
                Some(s) => s.as_str().to_string(),
                None => "".to_string() 
            };
            result.push_str(&(arg1 + arg2).to_string());
            match right {
                Some(s) => result.push_str(s.as_str()),
                None => ()
            }
            return evaluate_flat_expression(&result)
        },
        None => ()
    }

    match MULT_RE.captures(exp) {
        Some(captures) => {
            let (arg1, _operator, arg2, right): (i64, &str, i64, Option<regex::Match>) = (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str(),
                captures.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(4)
            );
            let mut result: String = (arg1 * arg2).to_string();
            match right {
                Some(s) => result.push_str(s.as_str()),
                None => ()
            }
            return evaluate_flat_expression(&result)
        },
        None => ()
    }

    panic!("unknown expr: {}", exp);
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
        assert_eq!(evaluate_flat_expression("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(evaluate_flat_expression("8 * 3 + 9 + 3 * 4 * 3"), 1440);
        assert_eq!(evaluate_flat_expression("11664 + 2 + 4 * 2"), 23340);
    }

    #[test]
    fn test_evaluate_exp() {
        assert_eq!(evaluate_expression("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(evaluate_expression("2 * 3 + (4 * 5)"), 46);
        assert_eq!(evaluate_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(evaluate_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(evaluate_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
    }
}