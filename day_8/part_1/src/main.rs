use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use lazy_static::lazy_static;
use regex::Regex;

const UNINITIATED: &str = "NONE";
const ACC: &str = "acc";
const JMP: &str = "jmp";
const NOP: &str = "nop";

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let n_instructions: usize = count_lines(open_file(filename));
    let aoc_reader = AocBufReader::from_file(open_file(filename));

    let mut instructions: Vec<Instruction> = parse_instructions(Box::new(aoc_reader), n_instructions);
    let return_val: i32 = debug_instructions(&mut instructions);
    println!("the final value is {}!", return_val);
}


fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}


fn count_lines(file: File) -> usize {
    BufReader::new(file).lines().count()
}


fn reset_instructions(instructions: &mut Vec<Instruction>) {
    for idx in 0..instructions.len() {
        instructions[idx].visited = false
    }
}


fn debug_instructions(instructions: &mut Vec<Instruction>) -> i32 {
    for idx in 0..instructions.len() {
        reset_instructions(instructions);
        let previous_instruction = instructions[idx].instruction;
        match previous_instruction {
            ACC => {continue;},
            JMP => {instructions[idx].instruction = NOP;},
            NOP => {instructions[idx].instruction = JMP;},
            _ => panic!("oh no! invalid instruction")
        }
        let found_value: i32 = execute_instructions(instructions);
        if found_value != -1 {
            return found_value
        }
        instructions[idx].instruction = previous_instruction;
    }
    -1
}


fn execute_instructions(instructions: &mut Vec<Instruction>) -> i32 {
    let mut accumulator: i32 = 0;
    let mut instruction_pointer: i32 = 0;
    while instruction_pointer >= 0 && (instruction_pointer as usize) < instructions.len() && !instructions[instruction_pointer as usize].visited {
        execute_instruction(instructions, &mut instruction_pointer, &mut accumulator);
    }
    if instruction_pointer as usize >= instructions.len() {
        return accumulator
    } else {
        return -1
    }
}


fn execute_instruction(instructions: &mut Vec<Instruction>, instruction_pointer: &mut i32, accumulator: &mut i32) {
    let current_instruction: &mut Instruction = &mut instructions[*instruction_pointer as usize];
    current_instruction.visited = true;
    match current_instruction.instruction {
        ACC => {
            *accumulator += current_instruction.val;
            *instruction_pointer += 1;
        },
        JMP => {
            *instruction_pointer += current_instruction.val;
        },
        NOP => {
            *instruction_pointer += 1;
        },
        _ => panic!("invalid instruction!")
    }
}


fn parse_instructions(lines: Box<dyn Iterator<Item = String>>, n_lines: usize) -> Vec<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^([a-z]{3}) ([+-]{1}[0-9]*)$"
        ).unwrap();
    }
    let mut instructions: Vec<Instruction> = vec![Instruction::fresh(); n_lines];
    
    for (idx, line) in lines.enumerate() {
        let (instruction, val): (&str, i32) = RE.captures(&line).and_then(
            | capture | Some((
                capture.get(1).unwrap().as_str(),
                capture.get(2).unwrap().as_str().parse::<i32>().unwrap()
            ))
        ).unwrap();
        instructions[idx].instruction = match instruction {
            "acc" => ACC,
            "jmp" => JMP,
            "nop" => NOP,
            _ => panic!("oh no!")
        };
        instructions[idx].val = val;
    }
    instructions
}


#[derive(Clone)]
struct Instruction {
    val: i32,
    instruction: &'static str,
    visited: bool
}


impl Instruction {
    fn fresh() -> Instruction {
        Instruction {
            val: 0,
            instruction: UNINITIATED,
            visited: false
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