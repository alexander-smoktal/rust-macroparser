use std::vec::IntoIter;

#[derive(Debug)]
pub struct Lexer {
    /// Input string
    input: IntoIter<u8>,
    /// Parsed characters, if parsed, but not accepted
    parsed: Vec<u8>,
    /// Currently popped chars
    backpack: Vec<u8>
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: Vec::from(input).into_iter(),
            parsed: vec![],
            backpack: vec![]
        }
    }

    pub fn next(&mut self) -> Option<u8> {
        let result: Option<u8>;
        
        if self.parsed.len() > 0 {
            result = self.parsed.pop()
        } else {
            result = self.input.next()
        }

        if let Some(c) = result {
            if c == ' ' as u8 {
                return self.next()
            }
            
            self.backpack.push(c)
        }

        result
    }

    pub fn accept(&mut self) {
        self.backpack.clear()
    }

    pub fn reject(&mut self) {
        self.parsed.append(&mut self.backpack);
        self.backpack.clear()
    }
}
