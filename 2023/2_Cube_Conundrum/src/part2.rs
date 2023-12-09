use std::{error::Error, fs};

use crate::common::{Parser, CubesSubset};

fn fewest_number_of_cubes_of_each_color(subsets: &Vec<CubesSubset>) -> CubesSubset {
    let blue = subsets.iter().map(|s| s.blue).max().unwrap();
    let red = subsets.iter().map(|s| s.red).max().unwrap();
    let green = subsets.iter().map(|s| s.green).max().unwrap();

    CubesSubset { blue, red, green, }
}

impl CubesSubset {
    pub fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

pub fn solution() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./day2.input")?;
    let mut parser = Parser::new();

    let sum = input.lines().fold(0, |acc, line| {
        let (_, subsets) = parser.parse(line).unwrap();
        let s = fewest_number_of_cubes_of_each_color(&subsets);

        acc + s.power()
    });

    println!("sum: {sum}");
    Ok(())
}
