use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let aoc_reader = AocBufReader::from_file(open_file(filename));

    let instructions: Vec<Instruction> = parse_instruction(Box::new(aoc_reader));
    navigate(&instructions);
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


fn navigate(instructions: &Vec<Instruction>) {
    let mut dyad = Dyad {
        turtle: Turtle { x: 0, y: 0 },
        waypoint: Waypoint { x: 10, y: 1 }
    };
    for instruction in instructions {
        dyad.execute(&instruction);
    }

    println!(
        "final position x: {}, y: {}, Manhattan distance: {}",
        dyad.turtle.x,
        dyad.turtle.y,
        dyad.turtle.x.abs() + dyad.turtle.y.abs()
    );
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


struct Forward {
    distance: i64
}


struct Instruction {
    translation: Option<Translation>,
    rotation: Option<Rotation>,
    forward: Option<Forward>
}


struct Turtle {
    x: i64,
    y: i64,
}


struct Waypoint {
    x: i64,
    y: i64
}


struct Dyad {
    turtle: Turtle,
    waypoint: Waypoint
}


impl Dyad {
    fn waypoint_relative_to_turtle(&self) -> (i64, i64) {
        (self.waypoint.x - self.turtle.x, self.waypoint.y - self.turtle.y)
    }


    fn rotate_waypoint(&mut self) {
        let (mut dx, mut dy) = self.waypoint_relative_to_turtle();
        /* rotation by 90 CW is a matrix multiplication
           is the linear transformation on R2:
           | 0, 1| |x|
           |-1, 0| |y|
        
           (x, y) -> (y, -x)  e.g. (-1, 1) -> (1, 1)
        */
        let tmp = dx;
        dx = dy;
        dy = -tmp;

        self.waypoint.x = self.turtle.x + dx;
        self.waypoint.y = self.turtle.y + dy;
    }


    fn move_to_waypoint(&mut self) {
        let (dx, dy) = self.waypoint_relative_to_turtle();

        self.turtle.x = self.waypoint.x;
        self.turtle.y = self.waypoint.y;

        self.waypoint.x = self.turtle.x + dx;
        self.waypoint.y = self.turtle.y + dy;
    }


    fn execute(&mut self, instruction: &Instruction) {
        match &instruction.rotation {
            Some(rotation) => {
                for _idx in 0..rotation.n_turns {
                    self.rotate_waypoint();
                }
            },
            None => ()
        }
        match &instruction.translation {
            Some(translation) => {
                self.waypoint.x = self.waypoint.x + translation.dx;
                self.waypoint.y = self.waypoint.y + translation.dy;
            },
            None => ()
        }
        match &instruction.forward {
            Some(forward) => {
                for _idx in 0..forward.distance {
                    self.move_to_waypoint();
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

    #[test]
    fn test_rotate() {
        let mut dyad: Dyad = Dyad {
            turtle: Turtle { x: 0, y: 0},
            waypoint: Waypoint { x: 1, y: 1}
        };
        assert_eq!((dyad.waypoint.x, dyad.waypoint.y), (1, 1));
        dyad.rotate_waypoint();
        assert_eq!((dyad.waypoint.x, dyad.waypoint.y), (1, -1));
        dyad.rotate_waypoint();
        assert_eq!((dyad.waypoint.x, dyad.waypoint.y), (-1, -1));
        dyad.rotate_waypoint();
        assert_eq!((dyad.waypoint.x, dyad.waypoint.y), (-1, 1));
    }

}