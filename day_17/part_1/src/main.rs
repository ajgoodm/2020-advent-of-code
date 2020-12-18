use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use itertools::Itertools;

const ARRAY_EDGE: usize = 48;
const SHIFT: usize = 24;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));

    let mut cube: CenteredCube = CenteredCube { data: [[[false; ARRAY_EDGE]; ARRAY_EDGE]; ARRAY_EDGE] };
    for (y_idx, line) in aoc_reader.enumerate() {
        for (x_idx, c) in line.chars().enumerate() {
            match c {
                '#' => cube.put(x_idx, y_idx, 0, true),
                '.' => (),
                _ => panic!("something went wrong!")
            }
        }
    }

    println!("{}", cube.data[24][24][24]);

    // for idx in 0..1 {
        // println!("step {}: {}", idx, cube.count_all());
        // cube.step();
    }
    println!("final count {}", cube.count_all());

}


struct CenteredCube {
    data: [[[bool; ARRAY_EDGE]; ARRAY_EDGE]; ARRAY_EDGE]
}

impl CenteredCube {
    fn put(&mut self, x: usize, y: usize, z: usize, val: bool) {
        let arr_y: usize = y + SHIFT;
        let arr_x: usize = x + SHIFT;
        let arr_z: usize = z + SHIFT;

        self.data[arr_z][arr_y][arr_x] = val;
    }

    fn step(&mut self) {
        let mut next = [[[false; ARRAY_EDGE]; ARRAY_EDGE]; ARRAY_EDGE];
        for z in 0..ARRAY_EDGE {
            for y in 0..ARRAY_EDGE {
                for x in 0..ARRAY_EDGE {
                    let val = self.data[z][y][x];
                    match val {
                        true => {
                            if self.count_active_neighbors(x, y, z) == 2 || self.count_active_neighbors(x, y, z) == 3 {
                                next[z][y][z] = true
                            }
                        },
                        false => {
                            if self.count_active_neighbors(x, y, z) == 3 {
                                next[z][y][x] = true
                            }
                        }
                    }
                }
            }
        }
        self.data = next;
    }

    fn count_active_neighbors(&self, x: usize, y: usize, z: usize) -> u8 {
        let x = x as i8;
        let y = y as i8;
        let z = z as i8;
        let mut n_neighbors: u8 = 0;
        for dx in -1i8..2 {
            for dy in -1i8..2 {
                for dz in -1i8..2 {
                    let x_arr = x + dx;
                    let y_arr = y + dy;
                    let z_arr = z + dz;
                    if (dx != 0 || dy != 0 || dz != 0)
                    && ( 0 <= x_arr && x_arr < ARRAY_EDGE as i8)
                    && ( 0 <= y_arr && y_arr < ARRAY_EDGE as i8)
                    && ( 0 <= z_arr && z_arr < ARRAY_EDGE as i8) {
                        println!("{}, {}, {}", dx, dy, dz);
                        if self.data[z_arr as usize][y_arr as usize][x_arr as usize] {n_neighbors += 1}
                    }

                }
            }
        }
        // if n_neighbors > 0 {
            // println!("{}, {}, {}", x + MIN_VAL, y + MIN_VAL, z + MIN_VAL);
            // println!("{}", n_neighbors);
        // }
        n_neighbors
    }

    fn count_all(&self) -> u64 {
        let mut count: u64 = 0;
        for z in 0..ARRAY_EDGE {
            for y in 0..ARRAY_EDGE {
                for x in 0..ARRAY_EDGE {
                    if self.data[z][y][x] {
                        println!("({}, {}, {})", z, y - 24, x - 24);
                        count += 1 
                    }
                }
            }
        }
        count
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
