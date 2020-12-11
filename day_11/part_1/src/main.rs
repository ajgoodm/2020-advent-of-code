use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));
    let mut grid: SeatingArea = SeatingArea::from_buffer(Box::new(aoc_reader));

    println!(
        "number of occupied seats at equilibrium: {}",
        run_simulation(grid)
    );
}

fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}

fn run_simulation(mut seating_area: SeatingArea) -> i64 {
    let mut previous_seating_area: SeatingArea = SeatingArea { grid: vec![vec![]] };
    while previous_seating_area != seating_area {
        previous_seating_area = SeatingArea::from_other(&seating_area);
        seating_area = seating_area.step();
    }

    seating_area.n_occupied()
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum SeatState {
    floor,
    empty,
    occupied
}


struct SeatingArea {
    grid: Vec<Vec<SeatState>>
}


impl SeatingArea {
    fn from_buffer(lines: Box<dyn Iterator<Item = String>>) -> SeatingArea {
        let mut grid: Vec<Vec<SeatState>> = vec![];
        for line in lines {
            let mut row: Vec<SeatState> = vec![];
            for c in line.chars() {
                match c {
                    '.' => row.push(SeatState::floor),
                    'L' => row.push(SeatState::empty),
                    '#' => row.push(SeatState::occupied),
                    _ => panic!("Error reading grid!")
                }
            }
            grid.push(row);
        }
        SeatingArea { grid: grid }
    }

    fn from_other(other: &SeatingArea) -> SeatingArea {
        SeatingArea { grid: other.grid.clone() }
    }

    fn n_rows(&self) -> usize {
        self.grid.len()
    }

    fn n_cols(&self) -> usize {
        if self.n_rows() == 0 {
            return 0
        }
        self.grid[0].len()
    }

    fn get_seat(&self, row: usize, col: usize) -> Option<&SeatState> {
        if (row >= self.n_rows()) || (col >= self.n_cols()) {
            return None;
        }
        Some(&self.grid[row][col])
    }

    fn get_adjacent_seats(&self, row: usize, col: usize) -> u16 {
        ((-1 as i8)..2).cartesian_product((-1 as i8)..2)
            .filter(|(drow, dcol)| { !(*drow == 0 && *dcol == 0) })
            .filter(|(drow, dcol)| { drow + row as i8 >= 0 && dcol + col as i8 >= 0 })
            .map( |(drow, dcol)| {
                let new_row: usize = (row as i8 + drow) as usize;
                let new_col: usize = (col as i8  + dcol) as usize;

                match self.get_seat(new_row, new_col) {
                    Some(seat) => match *seat {
                        SeatState::occupied => 1 as u16,
                        _ => 0 as u16
                    },
                    None => 0 as u16
                }
            }).sum()
    }

    fn _step_chair(&self, row: usize, col: usize) -> SeatState {
        match self.get_seat(row, col) {
            Some(seat) => match *seat {
                SeatState::floor => return SeatState::floor,
                SeatState::empty => {
                    if self.get_adjacent_seats(row, col) == 0 {
                        return SeatState::occupied
                    } else {
                        return SeatState::empty
                    }
                },
                SeatState::occupied => {
                    if self.get_adjacent_seats(row, col) >= 4 {
                        return SeatState::empty
                    } else {
                        return SeatState::occupied
                    }
                }
            },
            None => panic!("Don't step the void!")
        }
    }

    fn step(&self) -> SeatingArea {
        let mut new_grid: Vec<Vec<SeatState>> = vec![];
        for row in 0..self.n_rows() {
            let mut new_row: Vec<SeatState> = vec![];
            for col in 0..self.n_cols() {
                new_row.push(self._step_chair(row, col));
            }
            new_grid.push(new_row)
        }
        SeatingArea { grid: new_grid }
    }

    fn n_occupied(&self) -> i64 {
        (0..self.n_rows()).cartesian_product(0..self.n_cols())
            .map(|(row, col)| {
                match self.get_seat(row, col) {
                    Some(seat) => {
                        match *seat {
                            SeatState::occupied => 1,
                            _ => 0
                        }
                    },
                    None => panic!("don't count the void!")
                }
            }).sum::<i64>()
    }
}

impl PartialEq for SeatingArea {
    fn eq(&self, other: &Self) -> bool {
        if self.n_rows() != other.n_rows()
        && self.n_cols() != other.n_cols() {
            return false
        }
        for row in 0..self.n_rows() {
            for col in 0..self.n_cols() {
                if self.get_seat(row, col).unwrap() != other.get_seat(row, col).unwrap() {
                    return false
                }
            }
        }
        true
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
    fn test_step() {
        let grid: Vec<Vec<SeatState>> = vec![
            vec![SeatState::empty, SeatState::empty, SeatState::floor],
            vec![SeatState::empty, SeatState::empty, SeatState::floor],
            vec![SeatState::floor, SeatState::floor, SeatState::empty]
        ];


        /*
            LL.    ##.    ##.
            LL. -> ##. -> #..
            ..L    ..#    ..#
        */
        let seating_area = SeatingArea {grid: grid};
        let full_seats = seating_area.step();
        assert_eq!(full_seats.n_occupied(), 5);
        assert_eq!(full_seats.get_adjacent_seats(0, 0), 3);

        assert_eq!(seating_area.n_occupied(), 0);
        assert_eq!(seating_area.step().n_occupied(), 5);
        assert_eq!(seating_area.step().step().n_occupied(), 4);
        assert_eq!(seating_area.step().step().step().n_occupied(), 4);
    }
}