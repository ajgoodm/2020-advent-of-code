use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use itertools::Itertools;

const ARRAY_EDGE: usize = 48;
const HALF_ARRAY_EDGE: usize = 24;
const SHIFT: usize = 24;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));

    let mut cube = make_cube(Box::new(aoc_reader));
        
    for _idx in 0..6 {
        step(&mut cube);
    }


}

fn make_cube(boxed_iterator: Box<dyn Iterator< Item = String >>) -> [[[bool; ARRAY_EDGE]; ARRAY_EDGE]; ARRAY_EDGE] {
    let mut cube = [[[false; ARRAY_EDGE]; ARRAY_EDGE]; ARRAY_EDGE];
    for (y_idx, line) in boxed_iterator.enumerate() {
        for (x_idx, c) in line.chars().enumerate() {
            match c {
                '#' => cube[0 + HALF_ARRAY_EDGE][y_idx + HALF_ARRAY_EDGE][x_idx + HALF_ARRAY_EDGE] = true,
                '.' => (),
                _ => panic!("something went wrong!")
            }
        }
    }
    cube
}

fn count_all(cube: &[[[bool; ARRAY_EDGE]; ARRAY_EDGE]; ARRAY_EDGE]) -> u64 {
    (0usize..ARRAY_EDGE as usize)
        .cartesian_product(0usize..ARRAY_EDGE as usize)
        .cartesian_product(0usize..ARRAY_EDGE as usize)
        .map(
            |((z_coord, y_coord), x_coord)| {
                match cube[z_coord][y_coord][x_coord] {
                    true => {
                        if (z_coord == 0 || y_coord == 0 || x_coord == 0) {panic!("you're gonna need a bigger box")}
                        if (z_coord >= ARRAY_EDGE || y_coord >= ARRAY_EDGE || x_coord >= ARRAY_EDGE) {panic!("you're gonna need a bigger box")}
                        1
                    }
                    false => 0
                }
            }
        ).sum()
}

fn count_neighbors(
    cube: &[[[bool; ARRAY_EDGE]; ARRAY_EDGE]; ARRAY_EDGE],
    z_arr: usize,
    y_arr: usize,
    x_arr: usize
) -> u64 {    
    let count = (-1i32..2)
        .cartesian_product(-1i32..2)
        .cartesian_product(-1i32..2)
        .filter(|((dz, dy), dx)| { 
            (*dz != 0) || (*dy != 0) || (*dx != 0)
        })
        .map(|((dz, dy), dx)| { 
            (
                (z_arr as i32 + dz),
                (y_arr as i32 + dy),
                (x_arr as i32 + dx),
            )
        })
        .filter(|(z_arr, y_arr, x_arr)| {
            (*z_arr >= 0 && *z_arr < ARRAY_EDGE as i32)
            && (*y_arr >= 0 && *y_arr < ARRAY_EDGE as i32)
            && (*x_arr >= 0 && *x_arr < ARRAY_EDGE as i32)
        })
        .map(|(z_arr, y_arr, x_arr)| {
            (
                z_arr as usize,
                y_arr as usize,
                x_arr as usize
            )
        })
        .map (|(z_arr, y_arr, x_arr)| {
            match cube[z_arr][y_arr][x_arr] {
                true => 1,
                false => 0
            }
        }).sum();


    count
}


fn step(mut cube: &mut [[[bool; ARRAY_EDGE]; ARRAY_EDGE]; ARRAY_EDGE]) {
    let mut next = [[[false; ARRAY_EDGE]; ARRAY_EDGE]; ARRAY_EDGE];
    let _: () = (0usize..ARRAY_EDGE as usize)
        .cartesian_product(0usize..ARRAY_EDGE as usize)
        .cartesian_product(0usize..ARRAY_EDGE as usize)
        .map(
            |((z_coord, y_coord), x_coord)| {
                match cube[z_coord][y_coord][x_coord] {
                    true => {
                        if (count_neighbors(&cube, z_coord, y_coord, x_coord) == 2
                            || count_neighbors(&cube, z_coord, y_coord, x_coord) == 3) {
                                next[z_coord][y_coord][x_coord] = true
                            }
                    }
                    false => {
                        if count_neighbors(&cube, z_coord, y_coord, x_coord) == 3 {
                            next[z_coord][y_coord][x_coord] = true
                        }
                    }
                }
            }).collect();
    *cube = next;
    println!("after step: {}", count_all(&cube));
}

fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}


fn print_plane(cube: &[[[bool; ARRAY_EDGE]; ARRAY_EDGE]; ARRAY_EDGE], z_coord: usize) {
    for y_idx in 0usize..27 {
        let row: String = (24..ARRAY_EDGE).map(|x_idx| {
            match cube[z_coord][y_idx][x_idx] {
                true => '#',
                false => '.'
            }
        }).collect();
        println!("{}", row);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_neighbors() {

        /*
        24  .#.
        25  ..#
        26  ###
            222
            456
        */

        let aoc_reader = AocBufReader::from_file(open_file("../data/test_pt1_112.txt"));
        let mut cube = make_cube(Box::new(aoc_reader));

        assert_eq!(count_neighbors(&cube, 24, 26, 24), 1);
        assert_eq!(count_neighbors(&cube, 24, 26, 25), 3);
    }

}
