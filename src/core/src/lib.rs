pub mod math;
pub mod datastructures;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};

pub fn read_lines(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
