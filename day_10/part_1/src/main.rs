use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const MAX_ADAPTOR_JUMP: i64 = 3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));
    let mut adaptors: Vec<i64> = read_to_vec(Box::new(aoc_reader));

    let solution: i64 = find_jumps_product(&adaptors);
    println!("The solution is {}!", solution);

    let max_adaptor: i64 = *adaptors.iter().max().unwrap();
    adaptors.push(0);
    adaptors.sort();
    let mut cached_calls: HashMap<i64, i64> = HashMap::new();
    let n_permutations: i64 = find_valid_paths_to(&adaptors, max_adaptor, &mut cached_calls);
    println!("The number of valid permutations is {}!", n_permutations);
}

fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}

fn read_to_vec(lines: Box<dyn Iterator<Item = String>>) -> Vec<i64> {
    let mut adaptors: Vec<i64> = lines.map(
        |line| line.parse::<i64>().unwrap()
    ).collect();
    adaptors.sort();
    adaptors
}


fn find_valid_paths_to(adaptors: &Vec<i64>, paths_to: i64, cached_calls: &mut HashMap<i64, i64>) -> i64 {
    // A direct approach doesn't work; We're repeating a lot
    // of function calls, so we can cache them (there are only so many...)
    
    if paths_to == adaptors[0] {
        return 1;
    }

    adaptors.iter().filter(|val| {
        (**val < paths_to) && (paths_to - **val <= MAX_ADAPTOR_JUMP)
    }).map(
        |val| {
            if cached_calls.contains_key(val) {
                return cached_calls[val]
            } else {
                let result: i64 = find_valid_paths_to(
                    adaptors,
                    *val,
                    cached_calls
                );
                cached_calls.insert(*val, result);
                return result
            }
        }
    ).sum()
}




fn find_jumps_product(adaptors: &Vec<i64>) -> i64 {
    let mut one_jumps: i64 = 0;
    let mut three_jumps: i64 = 0;
    let mut prev_val: i64 = 0;

    for val in adaptors {
        if val - prev_val == 3 {
            three_jumps += 1;
        }
        if val - prev_val == 1 {
            one_jumps += 1;
        }
        prev_val = *val
    }

    // your device adaptor
    three_jumps = three_jumps + 1;

    one_jumps * three_jumps
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

struct Tree<T> {
    nodes: Vec<Node<T>>
}

impl<T> Tree<T> {
    fn new(root_val: T) -> Tree<T> {
        Tree {
            nodes: vec![
                Node {
                    parent: None,
                    first_child: None,
                    last_child: None,
                    data: root_val
                }
            ]
        }
    }

    fn get_node(&mut self, node_id: &NodeID) -> &mut Node<T> {
        &mut self.nodes[node_id.index]
    }

    fn add_node(&mut self, parent: &NodeID, data: T) -> NodeID {
        let new_node_id: NodeID = NodeID {
            index: self.nodes.len()
        };
        let new_node: Node<T> = Node {
            parent: Some(*parent),
            first_child: None,
            last_child: None,
            data: data
        };
        self.nodes.push(new_node);

        let parent_node = self.get_node(parent);
        match parent_node.last_child {
            Some(_node_id) => {},             
            None => {
                parent_node.first_child = Some(new_node_id);
            }
        };
        parent_node.last_child = Some(new_node_id);
        new_node_id
    }

    fn leaves(&self) -> Vec<NodeID> {
        self.nodes.iter().enumerate()
            .filter(|(_idx, node)| match node.first_child {None => true, Some(_node_id) => false})
            .map(|(idx, _node)| NodeID {index: idx}).collect()
    }
}

struct Node<T> {
    parent: Option<NodeID>,
    first_child: Option<NodeID>,
    last_child: Option<NodeID>,
    pub data: T
}

#[derive(Clone, Copy)]
struct NodeID {
    index: usize
}