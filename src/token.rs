use lazy_static::lazy_static;
use std::collections::HashMap;

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";
pub const NULL: &str = "NULL";

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
pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";

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
pub const IF: &str = "IF";
pub const RETURN: &str = "RETURN";
pub const ELSE: &str = "ELSE";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";

pub type Type = String;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, &'static str> = [
        ("fn", FUNCTION),
        ("let", LET),
        ("true", TRUE),
        ("false", FALSE),
        ("if", IF),
        ("else", ELSE),
        ("return", RETURN)
    ]
    .iter()
    .cloned()
    .collect();
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: Type,
    pub literal: String,
}

pub fn new(token_type: &str, literal: String) -> Token {
    Token {
        token_type: token_type.to_string(),
        literal,
    }
}

pub fn lookup_identifier(identifier: &str) -> &str {
    if let Some(v) = KEYWORDS.get(identifier) {
        return v;
    }

    IDENT
}
