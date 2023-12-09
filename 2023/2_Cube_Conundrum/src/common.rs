
#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
    InvalidNumber,
}

#[derive(Clone, Debug)]
pub struct Parser<'a> {
    pos: usize,
    len: usize,
    src: &'a str
}

#[derive(Debug)]
pub struct CubesSubset {
    pub blue: u32,
    pub red: u32,
    pub green: u32
}

impl CubesSubset {
    pub fn empty() -> Self {
        CubesSubset {
            blue: 0,
            red: 0,
            green: 0
        }
    }
}

pub type Result<T> = std::result::Result<T, ParserError>;

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser {
            pos: 0,
            src: "",
            len: 0
        }
    }

    pub fn parse(&mut self, input: &'a str) -> Result<(u32, Vec<CubesSubset>)> {
        self.src = input;
        self.pos = 0;
        self.len = input.chars().count();

        self.expect_and_consume("Game ")?;
        let id = self.consume_number()?;

        self.expect_and_consume(": ")?;

        let mut subsets = Vec::new();

        while let Some(subset) = self.consume_subset() {
            subsets.push(subset);
        }

        Ok((id, subsets))
    }

    fn consume_subset(&mut self) -> Option<CubesSubset> {
        let mut subset = CubesSubset::empty();

        while self.pos < self.len {
            self.skip_whitespaces();
            let num = match self.consume_number() {
                Ok(n) => n,
                Err(_) => return None
            };
            self.skip_whitespaces();
            let color = self.consume_until(|c| c == ',' || c == ';');

            match color {
                "blue" => subset.blue += num,
                "red" => subset.red += num,
                "green" => subset.green += num,
                _ => return None
            }

            match self.next_char() {
                Some(';') => return Some(subset),
                Some(',') => continue,
                Some(_) => return None,
                None => return Some(subset),
            }
        }

        None
    }

    fn next_char(&mut self) -> Option<char> {
        match self.src.chars().nth(self.pos) {
            Some(c) => {
                self.pos += 1;
                Some(c)
            },
            _ => None,
        }
    }

    fn expect_and_consume(&mut self, what: &str) -> Result<()> {
        let taken = self.src.chars().skip(self.pos).take(what.len()).collect::<String>();

        if taken != what {
            Err(ParserError::UnexpectedToken)
        }
        else {
            self.pos += taken.len();
            Ok(())
        }
    }

    fn consume_until(&mut self, condition: fn(char) -> bool) -> &str {
        let start = self.pos;
        while self.pos < self.len {
            let current_char = self.src.chars().nth(self.pos).unwrap();
            if condition(current_char) {
                break;
            }
            self.pos += 1;
        }
        &self.src[start..self.pos]
    }

    fn consume_number(&mut self) -> Result<u32> {
        let num = self.consume_until(|c| !c.is_numeric());

        if num.is_empty() {
            Err(ParserError::InvalidNumber)
        }
        else {
            Ok(num.parse::<u32>().unwrap())
        }
    }

    fn skip_whitespaces(&mut self) {
        self.consume_until(|c| c != ' ');
    }
}
