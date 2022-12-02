use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::instruction::{Instruction, Instruction1};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_instructions() -> Vec<(Instruction, bool)> {
    let mut instructions: Vec<(Instruction, bool)> = Vec::new();
    if let Ok(lines) = read_lines("instructions_test.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(' ').collect();
                let i;
                let operation = split[0];
                let number = split[1].parse().unwrap();
                if operation == "acc" {
                    i = Instruction::Acc(number);
                } else if operation == "jmp" {
                    i = Instruction::Jmp(number);
                } else {
                    i = Instruction::Nop(number);
                }
                instructions.push((i, false));
            }
        }
    }
    return instructions;
}

#[allow(dead_code)]
pub fn get_instructions1() -> Vec<(Instruction1, bool)> {
    let mut instructions: Vec<(Instruction1, bool)> = Vec::new();
    if let Ok(lines) = read_lines("instructions.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(' ').collect();
                let i;
                let operation = split[0];
                let number = split[1].parse().unwrap();
                if operation == "acc" {
                    i = Instruction1::Acc(number);
                } else if operation == "jmp" {
                    i = Instruction1::Jmp(number);
                } else {
                    i = Instruction1::Nop;
                }
                instructions.push((i, false));
            }
        }
    }
    return instructions;
}
