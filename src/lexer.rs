use crate::token;

pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: char
}

pub fn new(input: String) -> Lexer {
    let mut lex = Lexer{input, position: 0, read_position: 0, ch: '0'};
    lex.read_char();
    lex
}

impl Lexer {
    pub fn next_token(&mut self) -> token::Token {
        let token = match self.ch {
            '=' => token::new(token::ASSIGN, self.ch.to_string()),
            ';' => token::new(token::SEMICOLON, self.ch.to_string()),
            '(' => token::new(token::LPAREN, self.ch.to_string()),
            ')' => token::new(token::RPAREN, self.ch.to_string()),
            ',' => token::new(token::COMMA, self.ch.to_string()),
            '+' => token::new(token::PLUS, self.ch.to_string()),
            '{' => token::new(token::LBRACE, self.ch.to_string()),
            '}' => token::new(token::RBRACE, self.ch.to_string()),
            _ => token::new(token::EOF, String::from("0"))
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() as i32 {
            self.ch = '0';
        } else {
            self.ch = self.input.chars().nth(self.read_position as usize).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}
