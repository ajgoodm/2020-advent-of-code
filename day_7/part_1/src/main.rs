use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use regex::Regex;

const SHINY_GOLD: &str = "shiny gold";

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let aoc_buf_reader: AocBufReader = AocBufReader::from_file(open_file(filename));
    let bag_map = parse_bags(Box::new(aoc_buf_reader));
    let valid_bags = find_valid_bags(&bag_map, SHINY_GOLD);
    println!("{} different bags can hold your bag!", valid_bags.len());


    let mut bag_count: HashMap<String, i32> = HashMap::new();
    count_held_bags(&bag_map, &mut bag_count, SHINY_GOLD, 1);
    println!("you need a whopping {} total bags!", bag_count.values().sum::<i32>());
}


fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}


fn parse_line(line: &str) -> (String, Vec<(String, i32)>) {
    let parent_bag_re: Regex = Regex::new(
        r"^(.*) bags contain.*"
    ).unwrap();
    let parent_bag: String = parent_bag_re.captures(&line).and_then(
        | capture | Some(capture.get(1).unwrap().as_str())
    ).unwrap().to_string();

    let empty_bag_re: Regex = Regex::new(
        r"contain no other bags."
    ).unwrap();
    let non_empty_bag_re: Regex = Regex::new(
        "([0-9]+) ([^,0-9]*) bag"
    ).unwrap();


    let mut capacities: Vec<(String, i32)> = vec![];
    if empty_bag_re.captures(&line).iter().len() > 0 {
        return (parent_bag, capacities);
    } else {
        let captures = non_empty_bag_re.captures_iter(&line);
        for capture in captures {
            let size: i32 = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let color: &str = capture.get(2).unwrap().as_str();
            capacities.push((color.to_string(), size));
        }
    }

    (parent_bag, capacities)
}


fn parse_bags(lines: Box<dyn Iterator<Item = String>>) -> HashMap<String, HashMap<String, i32>> {
    let mut bag_map: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in lines {
        let (parent_bag_color, capacities) = parse_line(&line);
        bag_map.insert(parent_bag_color.to_string(), HashMap::new());
        
        for (child_bag_color, capacity) in capacities {
            if !bag_map.contains_key(&child_bag_color) {
                bag_map.insert(child_bag_color.to_string(), HashMap::new());
            } 
            bag_map.get_mut(&parent_bag_color).unwrap().insert(child_bag_color.to_string(), capacity);
        }
    }

    bag_map
}


fn add_bags(old_set: &mut HashSet<String>, new_bags: Vec<String>) {
    for new_bag in new_bags {
        old_set.insert(new_bag);
    }
}


fn find_valid_bags(bag_map: &HashMap<String, HashMap<String, i32>>, color: &str) -> HashSet<String> {
    let mut valid_bags: HashSet<String> = HashSet::new();

    for (bag_color, sub_bags) in bag_map {
        if sub_bags.contains_key(color) {
            valid_bags.insert(bag_color.to_string());
        }
    }

    let mut prior_len: usize = 0;
    while prior_len != valid_bags.len() {
        prior_len = valid_bags.len();
        let mut new_bags: Vec<String> = vec![];
        for valid_bag in &valid_bags {
            for (bag_color, sub_bags) in bag_map {
                if sub_bags.contains_key(valid_bag) {
                    new_bags.push(bag_color.to_string());
                }
            }
        }
        add_bags(&mut valid_bags, new_bags);
    }

    valid_bags
}


fn count_held_bags(bag_map: &HashMap<String, HashMap<String, i32>>, bag_count: &mut HashMap<String, i32>, color: &str, n_bags: i32) {
    for (parent_bag, capacity) in &bag_map[color] {    

        *bag_count.entry(parent_bag.to_string()).or_insert(0) += n_bags * *capacity;
        count_held_bags(bag_map, bag_count, &parent_bag, capacity * n_bags)
    }
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