#[derive(Debug)]
pub struct SymbolPosition(pub usize);

#[derive(Debug)]
pub struct NumberPosition {
    pub pos: usize,
    pub value: u32,
    pub len: usize,
}

#[derive(Debug)]
pub struct EngineView<'a> {
    pub src: Vec<&'a str>,
    pub nline: usize,
    pub nchar: usize,
}

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

    pub fn new(src: &'a str) -> Self {
        EngineView {
            src: src.lines().collect::<Vec<&str>>(),
            nline: 0,
            nchar: 0,
        }
    }
}
