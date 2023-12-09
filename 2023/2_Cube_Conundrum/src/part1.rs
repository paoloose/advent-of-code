use std::io;
use std::fs;

use crate::common::CubesSubset;
use crate::common::Parser;

static MAX_RED_CUBES: u32 = 12;
static MAX_GREEN_CUBES: u32 = 13;
static MAX_BLUE_CUBES: u32 = 14;

impl CubesSubset {
    pub fn is_valid(&self) -> bool {
        self.red <= MAX_RED_CUBES && self.green <= MAX_GREEN_CUBES && self.blue <= MAX_BLUE_CUBES
    }
}

pub fn solution() -> io::Result<()> {
    let input = fs::read_to_string("./day2.input")?;
    let mut parser = Parser::new();

    let sum = input.lines().fold(0, |acc, line| {
        let (id, subsets) = parser.parse(line).unwrap();

        if subsets.iter().all(|subset| subset.is_valid()) {
            acc + id
        }
        else {
            acc
        }
    });

    println!("sum: {sum}");

    Ok(())
}
