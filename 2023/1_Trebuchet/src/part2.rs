use std::error::Error;
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;

pub fn solution() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let digits_map: HashMap<Vec<u8>, char> = HashMap::from([
        (b"one".to_vec(), '1'),
        (b"two".to_vec(), '2'),
        (b"three".to_vec(), '3'),
        (b"four".to_vec(), '4'),
        (b"five".to_vec(), '5'),
        (b"six".to_vec(), '6'),
        (b"seven".to_vec(), '7'),
        (b"eight".to_vec(), '8'),
        (b"nine".to_vec(), '9'),
    ]);

    let sum = read_to_string("./day1.input")?.lines().fold(0, |acc, line| {
        let mut i = 1;
        let left = loop {
            if let Some(digit) = extract_digit(&line[0..i], &digits_map) {
                break digit;
            }
            i += 1;
        };

        let mut i = line.len() - 1;
        let right = loop {
            if let Some(digit) = extract_digit(&line[i..], &digits_map) {
                break digit;
            }
            i -= 1;
        };

        acc + format!("{}{}", left, right).parse::<u32>().unwrap()
    });

    let duration = start.elapsed();
    println!("{sum}");
    println!("time: {:?}", duration);

    Ok(())
}

fn extract_digit(line: &str, digits_map: &HashMap<Vec<u8>, char>) -> Option<char> {
    let line = line.as_bytes();

    let mut cursor_start = 0;
    let mut cursor_end = 1;

    while cursor_end != line.len() + 1 {
        if line[cursor_start].is_ascii_digit() {
           return Some(line[cursor_start] as char);
        }

        let accumulate = &line[cursor_start..cursor_end];

        if !digits_map.keys().any(|digit| digit.starts_with(accumulate)) {
            cursor_start = cursor_start + 1;
            cursor_end = cursor_start + 1;
            continue;
        }

        match digits_map.get(accumulate) {
            Some(digit) => return Some(*digit),
            None => cursor_end += 1,
        }
    }
    None
}
