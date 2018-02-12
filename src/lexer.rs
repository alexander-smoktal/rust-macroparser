#[derive(Debug)]
pub struct Lexer {
    /// Input string
    input: Vec<char>,
    /// Input position
    position: usize
}

type Position = usize;

impl Lexer {
    pub fn new(input: &str) -> Self {
        // Compiler bug. Can't drain string inplace
        let mut string = String::from(input);
        let vec = string.drain(..).collect();

        Lexer {
            input: vec,
            position: 0
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn next(&mut self) -> Option<char> {
        while let Some(result) = self.input.get(self.position).cloned() {
            self.position += 1;

            if result != ' ' {
                return Some(result)
            }
        }
        
        None
    }

    pub fn rollback(&mut self, position: Position) {
        self.position = position
    }
}
