use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use lazy_static::lazy_static;
use regex::Regex;


fn main() {
        let aoc_reader = AocBufReader::from_file(open_file("../data/input_rules.txt"));
        let input = parse_input(Box::new(aoc_reader));
        let mut matches: HashSet<String> = HashSet::new();

        // This is frustratingly necessary to get this to work with my
        // approach of dynamically building a regular expression. Regular
        // expressions have no sense of "counting", so I cannot match
        // "some number of pattern A followed by an equal number of pattern B"
        // for any number.
        for n_repeats in 1u32..50 {
            let rgx = make_complete_regex(0, &input, n_repeats); 
            let text_reader = AocBufReader::from_file(open_file("../data/input_text.txt"));
            for line in text_reader {
                if rgx.is_match(&line) {
                    matches.insert(line);
                }
            }        
        }

        println!("found {} total valid messages", matches.len());
}


struct Rule {
    next: Option<Vec<Vec<i64>>>,
    finally: Option<String>
}


fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}


fn make_complete_regex(start: i64, rules: &HashMap<i64, Rule>, n_repeats: u32) -> Regex {
    let mut rgx_str = "^".to_string();
    let r = &build_regex(start, rules, n_repeats);
    rgx_str.push_str(r);
    rgx_str.push('$');
    Regex::new(&rgx_str).unwrap()
}


fn build_regex(start: i64, rules: &HashMap<i64, Rule>, n_repeats: u32) -> String {
    let rule = rules.get(&start).unwrap();

    if start == 8 {
        let mut left: String = "(".to_string();
        left.push_str(
            &rule.next.as_ref().unwrap().iter()
                .map(|refs| {
                    refs.iter().map(|idx| build_regex(*idx, rules, n_repeats)).collect()
                }).collect::<Vec<String>>()
                .join("|")
        );
        left.push_str(")+");
        return left
    }

    if start == 11 {
        let mut left: String = "(".to_string();
        let single_rule = &rule.next.as_ref().unwrap()[0];
        left.push_str(
            &single_rule.iter()
                .map(|idx| {
                    let mut bit: String = build_regex(*idx, rules, n_repeats);
                    bit.push_str(&vec!["{","}"].join(&n_repeats.to_string()));
                    bit                
                }).collect::<Vec<String>>().join("")
        );
        left.push(')');
        return left
    }

    match &rule.finally {
        Some(c) => return c.to_string(),
        None => {
            let mut left: String = "(".to_string();
            left.push_str(
                &rule.next.as_ref().unwrap().iter()
                    .map(|refs| {
                        refs.iter().map(|idx| build_regex(*idx, rules, n_repeats)).collect()
                    }).collect::<Vec<String>>()
                    .join("|")
            );
            left.push(')');
            return left
        }
    }
}


fn parse_input(aoc_reader: Box<dyn Iterator< Item = String>>) -> HashMap<i64, Rule> {
    let mut input: HashMap<i64, Rule> = HashMap::new();
    for line in aoc_reader {
        let char_result = parse_char(&line);
        match char_result {
            Some((idx, char)) => {
                input.insert(
                    idx,
                    Rule {next: None, finally: Some(char.to_string())}
                );
            },
            None => {
                let (idx, vals) = parse_compound_rule(&line);
                input.insert(
                    idx,
                    Rule {next: Some(vals), finally: None}
                );
            }
        }
    }

    input
}


fn parse_char(line: &str) -> Option<(i64, &str)> {
    lazy_static! {
        static ref SINGLE_CHAR_RE: Regex = Regex::new(
            r#"^([0-9]+): "([a-z])"$"#
        ).unwrap();
    }
    SINGLE_CHAR_RE.captures(line)
        .map(
            |capture| {
                (
                    capture.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    capture.get(2).unwrap().as_str()
                )
            }
        )
}


fn parse_compound_rule(line: &str) -> (i64, Vec<Vec<i64>>) {
    lazy_static! {
        static ref COMPOUND_RULE_RE: Regex = Regex::new(
            r#"^([0-9]+): (.*)$"#
        ).unwrap();
    }
    COMPOUND_RULE_RE.captures(line)
        .map(
            |capture| {
                (
                    capture.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    capture.get(2).unwrap().as_str()
                        .split(" | ").map(|part| {
                            part.split(" ").map(|v| {
                                v.parse::<i64>().unwrap()
                            }).collect()
                        }).collect()
                )
            }
        ).unwrap()
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
    fn test_parse_compound_rule() {
        let (idx, rules) = parse_compound_rule("2: 1 3 | 3 1");
        assert_eq!(idx, 2);
        assert_eq!(rules, vec![vec![1, 3], vec![3, 1]]);
    }

    #[test]
    fn test_build_regex() {
        let aoc_reader = AocBufReader::from_file(open_file("../data/test_pt1.txt"));
        let input = parse_input(Box::new(aoc_reader));
        let rgx = build_regex(0, &input);

        println!("{}", rgx);
        let rgx: Regex = Regex::new(
            &rgx
        ).unwrap();
        assert!(rgx.is_match("aaaabb"));
        assert!(rgx.is_match("aaabab"));
        assert!(!rgx.is_match("b"));
    }
}