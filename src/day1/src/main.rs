mod dial_mod;
mod tests;
use crate::dial_mod::dial::Dial;
use crate::dial_mod::instruction::Instruction;

fn main() {
    const PATH: &str = "src/day1/input.txt";
    let lines = core::read_lines(PATH).unwrap();

    let instructions: Vec<_> = lines
        .map_while(Result::ok)
        .map(|x| Instruction::parse(&x).expect("Failed to parse instruction"))
        .collect();

    let mut dial = Dial::new(50);

    for instruction in instructions {
        dial.turn(&instruction);
    }

    println!("Zeros: {}", dial.zero_hits);
    println!("Zero wraps: {}", dial.zero_wraps);
}