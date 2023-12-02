use std::error::Error;
use std::fs::read_to_string;
use std::time::Instant;

pub fn solution() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let sum = read_to_string("./day1.input")?.lines().fold(0, |acc, line| {
        acc + parse_line(line)
    });

    let duration = start.elapsed();
    println!("{sum}");
    println!("time: {:?}", duration);

    Ok(())
}

#[inline]
fn parse_line(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter(|c| c.is_numeric()).collect::<Vec<char>>();

    format!(
        "{}{}",
        digits.first().unwrap(),
        digits.last().unwrap()
    ).parse::<u32>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day1() {
        let inputs = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ];
        let results = vec![12, 38, 15, 77];

        for (i, r) in inputs.iter().zip(results) {
            assert_eq!(parse_line(i), r);
        }
    }
}
