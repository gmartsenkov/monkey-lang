use crate::token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

pub fn new(input: String) -> Lexer {
    let mut lex = Lexer {
        input,
        position: 0,
        read_position: 0,
        ch: '0',
    };
    lex.read_char();
    lex
}

impl Lexer {
    #[allow(dead_code)]
    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => {
                if self.peek_ahead() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    token::new(token::EQ, ch.to_string() + &self.ch.to_string())
                } else {
                    token::new(token::ASSIGN, self.ch.to_string())
                }
            }
            '!' => {
                if self.peek_ahead() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    token::new(token::NOT_EQ, ch.to_string() + &self.ch.to_string())
                } else {
                    token::new(token::BANG, self.ch.to_string())
                }
            }
            ';' => token::new(token::SEMICOLON, self.ch.to_string()),
            '(' => token::new(token::LPAREN, self.ch.to_string()),
            ')' => token::new(token::RPAREN, self.ch.to_string()),
            ',' => token::new(token::COMMA, self.ch.to_string()),
            '+' => token::new(token::PLUS, self.ch.to_string()),
            '-' => token::new(token::MINUS, self.ch.to_string()),
            '*' => token::new(token::ASTERISK, self.ch.to_string()),
            '/' => token::new(token::SLASH, self.ch.to_string()),
            '<' => token::new(token::LT, self.ch.to_string()),
            '>' => token::new(token::GT, self.ch.to_string()),
            '{' => token::new(token::LBRACE, self.ch.to_string()),
            '}' => token::new(token::RBRACE, self.ch.to_string()),
            '\0' => token::new(token::EOF, String::from("")),
            _ => {
                if is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    let token_type = token::lookup_identifier(identifier);
                    return token::new(token_type, identifier.to_string());
                }
                if is_digit(self.ch) {
                    return token::new(token::INT, self.read_number().to_string());
                }
                token::new(token::ILLEGAL, self.ch.to_string())
            }
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;

        while is_letter(self.ch) {
            self.read_char();
        }

        self.input.get(position..self.position).unwrap()
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        self.input.get(position..self.position).unwrap()
    }

    fn peek_ahead(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }

        self.input.chars().nth(self.read_position).unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\n' || self.ch == '\t' || self.ch == '\r' {
            self.read_char();
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}
