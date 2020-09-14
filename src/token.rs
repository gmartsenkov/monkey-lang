use std::collections::HashMap;
use lazy_static::lazy_static;

pub const ILLEGAL: &Type = "ILLEGAL";
pub const EOF: &Type = "EOF";

// Identifiers + literals
pub const IDENT: &Type = "IDENT";
pub const INT: &Type = "INT";

// Operators
pub const ASSIGN: &Type = "=";
pub const PLUS: &Type = "+";
pub const MINUS: &Type = "-";
pub const BANG: &Type = "!";
pub const ASTERISK: &Type = "*";
pub const SLASH: &Type = "/";
pub const LT: &Type = "<";
pub const GT: &Type = ">";
pub const EQ: &Type = "==";
pub const NOT_EQ: &Type = "!=";

// Delimiters
pub const COMMA: &Type = ".";
pub const SEMICOLON: &Type = ";";
pub const LPAREN: &Type = "(";
pub const RPAREN: &Type = ")";
pub const LBRACE: &Type = "{";
pub const RBRACE: &Type = "}";

// Keywords
pub const FUNCTION: &Type = "FUNCTION";
pub const LET: &Type = "LET";
pub const IF: &Type = "IF";
pub const RETURN: &Type = "RETURN";
pub const ELSE: &Type = "ELSE";
pub const TRUE: &Type = "TRUE";
pub const FALSE: &Type = "FALSE";

pub type Type = str;

lazy_static! {
    static ref KEYWORDS : HashMap<&'static str, &'static str> = [
        ("fn", FUNCTION),
        ("let", LET),
        ("true", TRUE),
        ("false", FALSE),
        ("if", IF),
        ("else", ELSE),
        ("return", RETURN)
    ].iter().cloned().collect();
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type : &'a Type,
    pub literal : String,
}

pub fn new(token_type : &Type, literal : String) -> Token {
    Token{token_type, literal}
}

pub fn lookup_identifier(identifier : &str) -> &str {
    if let Some(v) = KEYWORDS.get(identifier) {
        return v;
    }

    IDENT
}
