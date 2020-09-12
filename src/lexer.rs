pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: char
}

pub fn new(input: String) -> Lexer {
    Lexer{input: input, position: 0, read_position: 0, ch: '0'}
}

impl Lexer {
    pub fn next_token(&self) -> &str {
       ""
    }
}
