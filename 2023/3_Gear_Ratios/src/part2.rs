use std::{error::Error, fs, ops::RangeInclusive};

use crate::common::{EngineView, SymbolPosition};

impl<'a> EngineView<'a> {
    fn next_gear(&mut self) -> Option<SymbolPosition> {
        let mut chars = self.src[self.nline].chars().skip(self.nchar);

        while let Some(c) = chars.next() {
            self.nchar += 1;
            if c == '*' {
                return Some(SymbolPosition(self.nchar - 1));
            }
        }

        None
    }
}

/// When non None, must guarantee that &line[range] is a valid number.
fn extract_num_at_pos(line: &str, x: usize) -> Option<std::ops::Range<usize>> {
    let sliced = line.as_bytes();

    if !sliced.get(x).map_or(false, u8::is_ascii_digit) {
        return None;
    }

    let mut growable_range = x..x + 1;

    loop {
        let left_is_numeric = sliced.get(growable_range.start - 1).map_or(false, u8::is_ascii_digit);
        let right_is_numeric = sliced.get(growable_range.end).map_or(false, u8::is_ascii_digit);

        if left_is_numeric {
            growable_range.start -= 1;
        }
        if right_is_numeric {
            growable_range.end += 1;
        }
        if !left_is_numeric && !right_is_numeric {
            break Some(growable_range);
        }
    }
}

fn extract_nums_from_slice(line: &str, range: RangeInclusive<usize>) -> Vec<u32> {
    let mut nums = vec![];
    let mut x = *range.start();
    let end = *range.end();

    while x < end {
        if let Some(ranged_num) = extract_num_at_pos(line, x) {
            let num = &line[ranged_num.clone()];
            nums.push(num.parse::<u32>().unwrap());
            x = ranged_num.end;
        }
        else {
            x += 1;
        }
    }

    nums
}

pub fn solution() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./day3.input")?;
    let mut engine = EngineView::new(&input);

    let nums = extract_nums_from_slice("...*..*....................328.........../...460.325.....................+...........*.........198.....-.316...............912..............", 1..=50);

    dbg!(nums);

    Ok(())
}
