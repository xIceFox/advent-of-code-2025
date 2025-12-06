mod dial;
mod tests;

use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::dial::dial::Dial;
use crate::dial::instruction::Instruction;

fn main() {
    const PATH: &str = "src/day1/input.txt";
    let file = File::open(PATH).expect(&*format!("File not found: {0}", PATH));
    let lines = BufReader::new(file).lines();

    let instructions: Vec<_> = lines
        .map_while(Result::ok)
        .map(|x| Instruction::parse(&*x).expect("Failed to parse instruction"))
        .collect();

    let mut dial = Dial::new(50);

    for instruction in instructions {
        dial.turn(&instruction);
    }

    println!("Zeros: {}", dial.zero_hits);
    println!("Zero wraps: {}", dial.zero_wraps);
}