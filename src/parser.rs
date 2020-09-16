use crate::{ast, lexer, token};
use std::collections::HashMap;

const LOWEST: u8 = 1;
const EQUALS: u8 = 2; // ==
const LESSGREATER: u8 = 3; // > or <
const SUM: u8 = 4; // +
const PRODUCT: u8 = 5; // *
const PREFIX: u8 = 6; // -X or !X
const CALL: u8 = 7; // myFunction(x)

type PrefixParseFn = fn(&Parser) -> ast::Expressions;
type InfixParseFn = fn(ast::Expressions) -> ast::Expressions;

pub struct Parser<'a> {
    lexer: &'a mut lexer::Lexer,
    current_token: token::Token,
    peek_token: token::Token,
    errors: Vec<String>,
    prefix_parse_functions: HashMap<token::Type, PrefixParseFn>,
    infix_parse_functions: HashMap<token::Type, InfixParseFn>,
}

pub fn new(lexer: &mut lexer::Lexer) -> Parser {
    let mut parser = Parser {
        lexer: lexer,
        current_token: token::new(token::NULL, "".to_string()),
        peek_token: token::new(token::NULL, "".to_string()),
        errors: vec![],
        prefix_parse_functions: HashMap::new(),
        infix_parse_functions: HashMap::new(),
    };

    parser.register_prefix_fn(token::IDENT.to_string(), parse_identifier);
    parser.next_token();
    parser.next_token();

    parser
}

fn parse_identifier(parser: &Parser) -> ast::Expressions {
    ast::Expressions::Identifier(ast::Identifier {
        token: parser.current_token.clone(),
        value: parser.current_token.literal.clone(),
    })
}

impl Parser<'_> {
    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program { statements: vec![] };

        while self.current_token.token_type != token::EOF {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }

            self.next_token()
        }

        program
    }

    fn parse_statement(&mut self) -> Option<ast::Statements> {
        match self.current_token.token_type.as_str() {
            token::LET => self.parse_let_statement(),
            token::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statements> {
        let current_token = self.current_token.clone();

        if !self.expect_peek_token(token::IDENT.to_string()) {
            return None;
        }

        let identifier = ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek_token(token::ASSIGN.to_string()) {
            return None;
        }

        while self.current_token.token_type == token::SEMICOLON {
            self.next_token();
        }

        Some(ast::Statements::Let(ast::LetStatement {
            token: current_token,
            name: identifier,
            value: ast::Expressions::Identifier(ast::Identifier {
                token: token::new(token::IDENT, "".to_string()),
                value: "".to_string(),
            }),
        }))
    }

    fn parse_return_statement(&mut self) -> Option<ast::Statements> {
        let statement = ast::ReturnStatement {
            token: self.current_token.clone(),
        };

        self.next_token();

        while self.current_token.token_type == token::SEMICOLON {
            self.next_token();
        }

        Some(ast::Statements::Return(statement))
    }

    fn parse_expression_statement(&mut self) -> Option<ast::Statements> {
        let current_token = self.current_token.clone();

        let expression = self.parse_expression(LOWEST);

        if self.peek_token.token_type == token::SEMICOLON {
            self.next_token();
        }

        if let Some(e) = expression {
            return Some(ast::Statements::Expression(ast::ExpressionStatement {
                expression: e,
                token: current_token,
            }));
        }

        None
    }

    fn parse_expression(&self, precedence: u8) -> Option<ast::Expressions> {
        let token_type = &self.current_token.token_type;
        if let Some(prefix) = self.prefix_parse_functions.get(token_type) {
            return Some(prefix(self));
        }
        None
    }

    fn parse_identifier(&self) -> ast::Expressions {
        ast::Expressions::Identifier(ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        })
    }

    fn register_prefix_fn(&mut self, token_type: token::Type, function: PrefixParseFn) {
        self.prefix_parse_functions.insert(token_type, function);
    }

    fn register_infix_fn(&mut self, token_type: token::Type, function: InfixParseFn) {
        self.infix_parse_functions.insert(token_type, function);
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().clone();
    }

    fn expect_peek_token(&mut self, expected: token::Type) -> bool {
        if self.peek_token.token_type == expected {
            self.next_token();
            return true;
        }

        let error = format!(
            "Parser Error: Expected {} but got {}",
            expected, self.peek_token.token_type
        );

        println!("{}", &error);
        self.errors.push(error);

        false
    }
}
