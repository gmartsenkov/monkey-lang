use std::collections::HashMap;
use lazy_static::lazy_static;

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";
pub const LT: &str = "<";
pub const GT: &str = ">";

// Delimiters
pub const COMMA: &str = ".";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";

lazy_static! {
    static ref KEYWORDS : HashMap<&'static str, &'static str> = [
        ("fn", FUNCTION),
        ("let", LET)
    ].iter().cloned().collect();
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type : &'a str,
    pub literal : String,
}

pub fn new(token_type : &str, literal : String) -> Token {
    Token{token_type, literal}
}

pub fn lookup_identifier(identifier : &str) -> &str {
    if let Some(v) = KEYWORDS.get(identifier) {
        return v;
    }

    IDENT
}
