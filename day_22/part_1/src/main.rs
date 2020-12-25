use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use lazy_static::lazy_static;
use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));
    let (deck_1, deck_2) = parse_input(Box::new(aoc_reader));
    println!("final score: {}", play_game(deck_1, deck_2));
}


fn play_game(mut deck_1: VecDeque<u64>, mut deck_2: VecDeque<u64>) -> u64 {
    loop {
        let card_1 = match deck_1.pop_front() {
            Some(card) => card,
            None => return score_game(deck_2)
        };
        let card_2 = match deck_2.pop_front() {
            Some(card) => card,
            None => {
                deck_1.push_front(card_1);
                return score_game(deck_1)
            }
        };

        if card_1 > card_2 {
            deck_1.push_back(card_1);
            deck_1.push_back(card_2);
        } else if card_2 > card_1 {
            deck_2.push_back(card_2);
            deck_2.push_back(card_1);
        } else { panic!("Oh no! duplicate card?"); }
    }
}


fn score_game(deck: VecDeque<u64>) -> u64 {
    deck.iter().rev().enumerate().map(
        | (idx, card) | {
            (idx + 1) as u64 * *card
        }
    ).sum()
}


fn parse_input(mut aoc_reader: Box<dyn Iterator< Item = String >>) -> (VecDeque<u64>, VecDeque<u64>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^([0-9]+)$"
        ).unwrap();
    }
    
    let (mut deck_1, mut deck_2): (VecDeque<u64>, VecDeque<u64>) = (VecDeque::new(), VecDeque::new());
    let _ = aoc_reader.next();
    loop {
        match RE.captures(&aoc_reader.next().unwrap()) {
            Some(capture) => deck_1.push_back(capture.get(1).unwrap().as_str().parse::<u64>().unwrap()),
            None => break
        }
    }

    let _ = aoc_reader.next();
    for line in aoc_reader {
        match RE.captures(&line) {
            Some(capture) => deck_2.push_back(capture.get(1).unwrap().as_str().parse::<u64>().unwrap()),
            None => break
        }
    }
    (deck_1, deck_2)
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
