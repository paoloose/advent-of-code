use std::error::Error;

// Day 1 🐢
// --------
// We use .fold over .sum to avoid having to collect the iterator into a Vec

pub mod part1;
pub mod part2;

fn main() -> Result<(), Box<dyn Error>> {
    part1::solution()?;
    part2::solution()?;

    Ok(())
}
