use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut aoc_buf_reader: AocBufReader = AocBufReader::from_file(open_file(filename));
    let total_sum: i32 = aoc_buf_reader.map(|lines| count_chars(&lines)).sum();
    println!("Unique questions answered yes: {}", total_sum);

    aoc_buf_reader = AocBufReader::from_file(open_file(filename));
    let total_shared: i32 = aoc_buf_reader.map(|lines| count_shared_chars(&lines)).sum();
    println!("Shared questions answered yes: {}", total_shared);

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
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lines = vec![];

        loop {
            match self.iter.next() {
                Some(result) => match result {
                    Ok(line) => {
                        if line == "" {
                            break Some(lines)
                        } else {
                            lines.push(line)
                        }
                    },
                    Err(error) => panic!(error)
                },
                None => {
                    if lines.len() != 0 {
                        break Some(lines)
                    } else {
                        break None
                    }
                }
            }
        }
    }
}


fn count_chars(lines: &Vec<String>) -> i32 {
    let mut chars: HashSet<char> = HashSet::new();
    for line in lines {
        for c in line.chars() {
            chars.insert(c);
        }
    }
    chars.len() as i32
}


fn count_shared_chars(lines: &Vec<String>) -> i32 {
    let n_passengers: i32 = lines.len() as i32;

    let mut chars: HashMap<char, i32> = HashMap::new();
    for line in lines {
        for c in line.chars() {
            *chars.entry(c).or_insert(0) += 1;
        }
    }

    chars.iter().map(
        |(c, n)| {
            if *n == n_passengers {
                return 1
            }
            return 0
        }
    ).sum()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars(&vec!["abc".to_string()]), 3);
        assert_eq!(count_chars(&vec!["a".to_string(), "bc".to_string()]), 3);
        assert_eq!(count_chars(&vec!["a".to_string(), "a".to_string()]), 1);
    }

    #[test]
    fn test_count_shared_chars() {
        assert_eq!(count_shared_chars(&vec!["abc".to_string()]), 3);
        assert_eq!(count_shared_chars(&vec!["a".to_string(), "bc".to_string()]), 0);
        assert_eq!(count_shared_chars(&vec!["a".to_string(), "a".to_string()]), 1);
    }
}

