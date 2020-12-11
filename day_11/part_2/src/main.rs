use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use itertools::Itertools;

const DIRECTIONS: [(i8, i8); 9] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 0), (0, 1),
    (1, -1), (1, 0), (1, 1)
];

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

    fn get_visible_seats(&self, row: usize, col: usize) -> i16 {
        DIRECTIONS.iter()
            .filter(|(drow, dcol)| !(*drow == 0 && *dcol == 0))
            .map(|(drow, dcol)| {
            let (mut check_row, mut check_col) = (row as i8 + drow, col as i8 + dcol);
            let mut val = 2;
            while val ==2 {
                val = match self.get_seat(check_row as usize, check_col as usize) {
                    Some(seat) => match *seat {
                        SeatState::empty => 0,
                        SeatState::occupied => 1,
                        SeatState::floor => 2
                    },
                    None => 0
                };

                check_row = check_row + drow;
                check_col = check_col + dcol;
            }
            val
        }).sum::<i16>()
    }

    fn _step_chair(&self, row: usize, col: usize) -> SeatState {
        match self.get_seat(row, col) {
            Some(seat) => match *seat {
                SeatState::floor => return SeatState::floor,
                SeatState::empty => {
                    if self.get_visible_seats(row, col) == 0 {
                        return SeatState::occupied
                    } else {
                        return SeatState::empty
                    }
                },
                SeatState::occupied => {
                    if self.get_visible_seats(row, col) >= 5 {
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
    fn test_n_visible() {
        let grid: Vec<Vec<SeatState>> = vec![
            vec![SeatState::occupied, SeatState::empty, SeatState::floor],
            vec![SeatState::empty, SeatState::occupied, SeatState::floor],
            vec![SeatState::floor, SeatState::floor, SeatState::occupied]
        ];


        /*
            # L .
            L # .
            . . #
        */
        let seating_area = SeatingArea {grid: grid};
        assert_eq!(seating_area.get_visible_seats(0, 0), 1);
        assert_eq!(seating_area.get_visible_seats(0, 1), 2);
        assert_eq!(seating_area.get_visible_seats(0, 2), 2);

    }
}