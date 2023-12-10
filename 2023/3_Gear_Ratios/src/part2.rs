use std::{error::Error, fs, ops::RangeInclusive};

use crate::common::{EngineView, SymbolPosition};

impl<'a> EngineView<'a> {
    fn next_gear(&mut self) -> Option<SymbolPosition> {
        if self.src.is_empty() { return None; }
        let mut chars = self.src[self.nline].chars().skip(self.nchar);

        loop {
            match chars.next() {
                Some(c) => {
                    self.nchar += 1;
                    if c == '*' {
                        return Some(SymbolPosition(self.nchar - 1));
                    }
                },
                None => {
                    self.nline += 1;
                    self.nchar = 0;
                    if self.nline == self.src.len() { return None; }
                    chars = self.src[self.nline].chars().skip(0);
                },
            }
        }
    }

    fn line_up(&self) -> Option<&str> {
        if self.src.is_empty() { return None }
        if self.nline == 0 { return None }
        Some(self.src[self.nline - 1])
    }

    fn line_down(&self) -> Option<&str> {
        if self.src.is_empty() { return None }
        if self.nline == self.src.len() - 1 { return None }
        Some(self.src[self.nline + 1])
    }

    fn current_line(&self) -> Option<&str> {
        if self.src.is_empty() { return None }
        Some(self.src[self.nline])
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

    while x <= end {
        // println!("extracting at pos {x} -> {c}", c=line.as_bytes()[x] as char);
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
    let mut sum = 0;

    while let Some(gear) = engine.next_gear() {
        let range = gear.0.checked_sub(1).unwrap_or(0)..=gear.0.checked_add(1).unwrap_or(gear.0);

        let nums_up = engine.line_up()
            .map_or(vec![], |l| extract_nums_from_slice(l, range.clone()));
        let nums_curr = engine.current_line()
            .map_or(vec![], |l| extract_nums_from_slice(l, range.clone()));
        let nums_down = engine.line_down()
            .map_or(vec![], |l| extract_nums_from_slice(l, range.clone()));

        let nums = [nums_up, nums_curr, nums_down].concat();
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
