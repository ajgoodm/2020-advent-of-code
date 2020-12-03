use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const TREE: char = '#';
const SNOW: char = '.';


struct Point {
    row: usize,
    col: usize
}


struct TreeMap {
    map: Vec<Vec<bool>>
}


impl TreeMap {
    fn from_buffer(read_buffer: BufReader<File>) -> TreeMap {
        let mut map: Vec<Vec<bool>> = vec![];
        for line in read_buffer.lines() {
            let line = line.expect("Could not read line!");
            let mut row: Vec<bool> = vec![];

            for c in line.chars() {
                if c == TREE {
                    row.push(true);
                } else if c == SNOW {
                    row.push(false);
                } else {
                    panic!("Neither tree nor now!");
                }
            }
            map.push(row);
        }

        TreeMap { map: map }
    }

    fn _get(&self, row: usize, col: usize) -> bool{
        self.map.get(row).unwrap().get(col).unwrap().clone()
    }

    fn n_rows(&self) -> usize {
        self.map.len()
    }

    fn n_cols(&self) -> usize {
        self.map.get(0).unwrap().len()
    }

    fn get(&self, position: &Point) -> bool {
        let (row, col) = (position.row, position.col);

        if row >= self.n_rows() {
            panic!("You fell off the map!");
        }
        let col = col % self.n_cols();
        self._get(row, col).clone()
    }

    fn count_trees(&self, slope_row: usize, slope_col: usize) -> i32 {
        let mut position: Point = Point {
            row: 0,
            col: 0
        };
        let mut n_trees: i32 = 0;
        while position.row < self.n_rows() {
            if self.get(&position) {
                n_trees = n_trees + 1;
            }
            position.row = position.row + slope_row;
            position.col = position.col + slope_col;
        }
        n_trees
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let slope_row: usize = args[2].parse::<usize>().unwrap();
    let slope_col: usize = args[3].parse::<usize>().unwrap();

    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let tree_map: TreeMap = TreeMap::from_buffer(f);
    let n_trees = tree_map.count_trees(slope_row, slope_col);
    println!("you hit {} trees!", n_trees);
}
