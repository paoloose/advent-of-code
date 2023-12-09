use std::error::Error;

// Day 3 🐢
// --------

pub mod part1;
pub mod part2;
pub mod common;

fn main() -> Result<(), Box<dyn Error>> {
    part1::solution()?;
    part2::solution()?;

    Ok(())
}
