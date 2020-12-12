use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));

    let mut turtle: Turtle = Turtle { x: 0, y: 0, orientation: Orientation::E };
    let instructions: Vec<Instruction> = parse_instruction(Box::new(aoc_reader));
    for instruction in instructions {
        turtle.execute(&instruction);
    }

    println!("Final position x: {}, y: {}, Manhattan dist: {}", turtle.x, turtle.y, turtle.x.abs() + turtle.y.abs());
}

fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}

fn parse_instruction(lines: Box<dyn Iterator<Item = String>>) -> Vec<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^([NESWLRF]{1})([0-9]*)$"
        ).unwrap();
    }

    let mut instructions: Vec<Instruction> = vec![];
    for line in lines {
        let (instruction_str, val): (&str, i64) = RE.captures(&line).and_then(
            |capture| Some((
                capture.get(1).unwrap().as_str(),
                capture.get(2).unwrap().as_str().parse::<i64>().unwrap()
            ))
        ).unwrap();
        instructions.push(
            match instruction_str {
                "N" => Instruction { translation: Some(Translation { dx: 0, dy: val }), rotation: None, forward: None },
                "E" => Instruction { translation: Some(Translation { dx: val, dy: 0 }), rotation: None, forward: None },
                "S" => Instruction { translation: Some(Translation { dx: 0, dy: -val }), rotation: None, forward: None },
                "W" => Instruction { translation: Some(Translation { dx: -val, dy: 0 }), rotation: None, forward: None },
                "R" => Instruction { translation: None, rotation: Some(Rotation::from_input("R", val)), forward: None },
                "L" => Instruction { translation: None, rotation: Some(Rotation::from_input("L", val)), forward: None },
                "F" => Instruction { translation: None, rotation: None, forward: Some(Forward { distance: val }) },
                _ => panic!("Unimplemented instruction")
            }
        )
    }

    instructions
}


struct Translation {
    dx: i64,
    dy: i64
}


struct Rotation {
    // clockwise
    n_turns: u8
}


impl Rotation {
    fn from_input(l_or_r: &str, val: i64) -> Rotation {
        let pos_or_neg: i16 = match l_or_r {
            "R" => 1,
            "L" => -1,
            _ => panic!("can only create rotations from  L or R")
        };
        let mut n_turns: i16 = pos_or_neg * (val as i16 / 90);
        if n_turns < 0 {
            n_turns = n_turns + 4;
        }
        if n_turns >= 4 {
            n_turns = n_turns - 4;
        }
        Rotation { n_turns: n_turns as u8 }
    }
}


enum Orientation {
    N,
    E,
    S,
    W
}


struct Forward {
    distance: i64
}


struct Instruction {
    translation: Option<Translation>,
    rotation: Option<Rotation>,
    forward: Option<Forward>
}


impl Orientation {
    fn turn(&self) -> Orientation {
        match self {
            Orientation::N => Orientation::E,
            Orientation::E => Orientation::S,
            Orientation::S => Orientation::W,
            Orientation::W => Orientation::N
        }
    }
}


struct Turtle {
    x: i64,
    y: i64,
    orientation: Orientation
}


impl Turtle {
    fn execute(&mut self, instruction: &Instruction) {
        match &instruction.rotation {
            Some(rotation) => {
                for _idx in 0..rotation.n_turns {
                    self.orientation = self.orientation.turn();
                }
            },
            None => ()
        }
        match &instruction.translation {
            Some(translation) => {
                self.x = self.x + translation.dx;
                self.y = self.y + translation.dy;
            },
            None => ()
        }
        match &instruction.forward {
            Some(forward) => {
                match self.orientation {
                    Orientation::N => self.y = self.y + forward.distance,
                    Orientation::E => self.x = self.x + forward.distance,
                    Orientation::S => self.y = self.y - forward.distance,
                    Orientation::W => self.x = self.x - forward.distance
                }
            }
            None => ()
        }
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
    fn test_rotation_parse() {
        assert_eq!(Rotation::from_input("L", 0).n_turns, 0);
        assert_eq!(Rotation::from_input("L", 90).n_turns, 3);
        assert_eq!(Rotation::from_input("L", 180).n_turns, 2);
        assert_eq!(Rotation::from_input("L", 270).n_turns, 1);
        assert_eq!(Rotation::from_input("L", 360).n_turns, 0);
        assert_eq!(Rotation::from_input("R", 0).n_turns, 0);
        assert_eq!(Rotation::from_input("R", 90).n_turns, 1);
        assert_eq!(Rotation::from_input("R", 180).n_turns, 2);
        assert_eq!(Rotation::from_input("R", 270).n_turns, 3);
        assert_eq!(Rotation::from_input("R", 360).n_turns, 0);
    }

}