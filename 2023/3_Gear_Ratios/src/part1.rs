use std::{error::Error, fs};

#[derive(Debug)]
struct SymbolPosition(usize);

#[derive(Debug)]
struct NumberPosition {
    pub pos: usize,
    pub value: u32,
    pub len: usize,
}

#[derive(Debug)]
struct EngineView<'a> {
    src: Vec<&'a str>,
    nline: usize,
    nchar: usize,
}

static IS_SYMBOL_CHAR: fn(char) -> bool = |c: char| !c.is_numeric() && c != '.';

impl<'a> EngineView<'a> {
    pub fn next_num(&mut self) -> Option<NumberPosition> {
        if self.src.is_empty() {
            return None;
        }
        let chars = &mut self.src[self.nline].chars().skip(self.nchar);
        loop {
            match &chars.next() {
                Some(c @ '0'..='9') => {
                    let mut num = String::new();
                    num.push(*c);
                    loop {
                        match chars.next() {
                            Some(c @ '0'..='9') => num.push(c),
                            _ => break,
                        }
                    }
                    self.nchar += num.len();
                    return Some(NumberPosition {
                        pos: self.nchar - num.len(),
                        len: num.len(),
                        value: num.parse::<u32>().unwrap(),
                    });
                },
                Some(_) => self.nchar += 1,
                None => return None,
            }
        }
    }

    fn new(src: &'a str) -> Self {
        EngineView {
            src: src.lines().collect::<Vec<&str>>(),
            nline: 0,
            nchar: 0,
        }
    }

    pub fn advance_and_sum(&mut self) -> Option<u32> {
        if self.src.is_empty() { return None; }
        if self.nline == self.src.len() { return None; }
        let mut sum = 0;

        while let Some(num) = self.next_num() {
            let mut contains;

            let roffset = if self.nchar == self.src[self.nline].len() { 1 } else { 0 };
            let loffset = if num.pos > 0 { 1 } else { 0 };

            let slice_range = num.pos - loffset..=num.pos + num.len - roffset;

            let slice_curr = &self.src[self.nline][slice_range.clone()];
            contains = slice_curr.contains(IS_SYMBOL_CHAR);

            if self.nline > 0 {
                let slice_up = &self.src[self.nline - 1][slice_range.clone()];
                if slice_up.contains(IS_SYMBOL_CHAR) {
                    contains = true;
                }
            }
            if self.nline < self.src.len() - 1 {
                let slice_down = &self.src[self.nline + 1][slice_range];
                if slice_down.contains(IS_SYMBOL_CHAR) {
                    contains = true;
                }
            }

            if contains {
                sum += num.value;
            }
        }
        self.nline += 1;
        self.nchar = 0;
        Some(sum)
    }
}

pub fn solution() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./day3.input")?;
    let mut engine = EngineView::new(&input);

    let mut total_sum = 0;
    while let Some(sum) = engine.advance_and_sum() {
        total_sum += sum;
    }

    println!("sum: {}", total_sum);
    Ok(())
}
