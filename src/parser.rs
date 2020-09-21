use crate::{ast, lexer, token};
use lazy_static::lazy_static;
use std::collections::HashMap;

const LOWEST: u8 = 1;
const EQUALS: u8 = 2; // ==
const LESSGREATER: u8 = 3; // > or <
const SUM: u8 = 4; // +
const PRODUCT: u8 = 5; // *
const PREFIX: u8 = 6; // -X or !X
const CALL: u8 = 7; // myFunction(x)

type PrefixParseFn = fn(&mut Parser) -> Option<ast::Expressions>;
type InfixParseFn = fn(ast::Expressions, &mut Parser) -> Option<ast::Expressions>;

lazy_static! {
    static ref PRECEDENTS: HashMap<token::Type, u8> = {
        let mut m: HashMap<token::Type, u8> = HashMap::new();
        m.insert(token::EQ.to_string(), EQUALS);
        m.insert(token::NOT_EQ.to_string(), EQUALS);
        m.insert(token::LT.to_string(), LESSGREATER);
        m.insert(token::GT.to_string(), LESSGREATER);
        m.insert(token::PLUS.to_string(), SUM);
        m.insert(token::MINUS.to_string(), SUM);
        m.insert(token::SLASH.to_string(), PRODUCT);
        m.insert(token::ASTERISK.to_string(), PRODUCT);
        m
    };
}

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
    parser.register_prefix_fn(token::INT.to_string(), parse_integer_literal);
    parser.register_prefix_fn(token::MINUS.to_string(), parse_prefix_expression);
    parser.register_prefix_fn(token::BANG.to_string(), parse_prefix_expression);
    parser.register_prefix_fn(token::TRUE.to_string(), parse_boolean_expression);
    parser.register_prefix_fn(token::FALSE.to_string(), parse_boolean_expression);
    parser.register_prefix_fn(token::LPAREN.to_string(), parse_grouped_expression);
    parser.register_prefix_fn(token::IF.to_string(), parse_if_expression);

    parser.register_infix_fn(token::PLUS.to_string(), parse_infix_expression);
    parser.register_infix_fn(token::MINUS.to_string(), parse_infix_expression);
    parser.register_infix_fn(token::SLASH.to_string(), parse_infix_expression);
    parser.register_infix_fn(token::ASTERISK.to_string(), parse_infix_expression);
    parser.register_infix_fn(token::EQ.to_string(), parse_infix_expression);
    parser.register_infix_fn(token::NOT_EQ.to_string(), parse_infix_expression);
    parser.register_infix_fn(token::LT.to_string(), parse_infix_expression);
    parser.register_infix_fn(token::GT.to_string(), parse_infix_expression);

    parser.next_token();
    parser.next_token();

    parser
}

fn parse_if_expression(parser: &mut Parser) -> Option<ast::Expressions> {
    let token = parser.current_token.clone();

    if !parser.expect_peek_token(token::LPAREN) {
        return None;
    }

    parser.next_token();

    let condition;

    if let Some(i) = parser.parse_expression(LOWEST) {
        condition = i;
    } else {
        return None;
    };

    if !parser.expect_peek_token(token::RPAREN) {
        return None;
    }

    if !parser.expect_peek_token(token::LBRACE) {
        return None;
    }

    let consequence = parser.parse_block_statement();
    let mut alternative = None;

    if parser.peek_token.token_type == token::ELSE {
        parser.next_token();

        if !parser.expect_peek_token(token::LBRACE) {
            return None;
        }

        alternative = Some(parser.parse_block_statement());
    }

    Some(ast::Expressions::If(ast::IfStatement{
        token,
        condition: Box::new(condition),
        consequence,
        alternative
    }))
}

fn parse_grouped_expression(parser: &mut Parser) -> Option<ast::Expressions> {
    parser.next_token();

    let expression = parser.parse_expression(LOWEST);

    if !parser.expect_peek_token(token::RPAREN) {
        return None;
    }

    expression
}

fn parse_identifier(parser: &mut Parser) -> Option<ast::Expressions> {
    Some(ast::Expressions::Identifier(ast::Identifier {
        token: parser.current_token.clone(),
        value: parser.current_token.literal.clone(),
    }))
}

fn parse_integer_literal(parser: &mut Parser) -> Option<ast::Expressions> {
    if let Ok(v) = parser.current_token.literal.parse::<i64>() {
        return Some(ast::Expressions::IntegerLiteral(ast::IntegerLiteral {
            token: parser.current_token.clone(),
            value: v,
        }));
    }
    None
}

fn parse_boolean_expression(parser: &mut Parser) -> Option<ast::Expressions> {
    Some(ast::Expressions::Boolean(ast::Boolean {
        token: parser.current_token.clone(),
        value: parser.current_token.token_type == token::TRUE,
    }))
}

fn parse_prefix_expression(parser: &mut Parser) -> Option<ast::Expressions> {
    let current_token = parser.current_token.clone();

    parser.next_token();

    let expression = parser.parse_expression(PREFIX);

    if let Some(e) = expression {
        return Some(ast::Expressions::Prefix(ast::PrefixExpression {
            token: current_token.clone(),
            operator: current_token.literal,
            right: Box::new(e),
        }));
    }

    None
}

fn parse_infix_expression(left: ast::Expressions, parser: &mut Parser) -> Option<ast::Expressions> {
    let current_token = parser.current_token.clone();

    let precedence = parser.current_precedence();

    parser.next_token();

    if let Some(right) = parser.parse_expression(precedence) {
        return Some(ast::Expressions::Infix(ast::InfixExpression {
            token: current_token.clone(),
            operator: current_token.literal,
            left: Box::new(left),
            right: Box::new(right),
        }));
    }

    None
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

        if !self.expect_peek_token(token::IDENT) {
            return None;
        }

        let identifier = ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek_token(token::ASSIGN) {
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

    fn parse_expression(&mut self, precedence: u8) -> Option<ast::Expressions> {
        let token_type = &self.current_token.token_type;
        if let Some(prefix) = self.prefix_parse_functions.get(token_type) {
            let mut left_expresion = prefix(self);

            while self.peek_token.token_type != token::SEMICOLON
                && precedence < self.peek_precedence()
            {
                if let Some(&inflix) = self.infix_parse_functions.get(&self.peek_token.token_type) {
                    self.next_token();

                    left_expresion = inflix(left_expresion.unwrap(), self);
                }
            }
            return left_expresion;
        }

        let error = format!(
            "Parser error: No prefix parse function found for {}",
            self.current_token.token_type
        );
        println!("{}", error);
        self.errors.push(error);
        None
    }

    fn parse_identifier(&self) -> ast::Expressions {
        ast::Expressions::Identifier(ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        })
    }

    fn parse_block_statement(&mut self) -> ast::BlockStatement {
        let token = self.current_token.clone();
        let mut statements = Vec::new();

        while self.current_token.token_type != token::RBRACE && self.current_token.token_type != token::EOF {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }

            self.next_token()
        }

        ast::BlockStatement{token, statements}
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

    fn peek_precedence(&self) -> u8 {
        if let Some(&precendence) = PRECEDENTS.get(&self.peek_token.token_type) {
            return precendence;
        }
        LOWEST
    }

    fn current_precedence(&self) -> u8 {
        if let Some(&precendence) = PRECEDENTS.get(&self.current_token.token_type) {
            return precendence;
        }
        LOWEST
    }

    fn expect_peek_token(&mut self, expected: &str) -> bool {
        if self.peek_token.token_type == expected.to_string() {
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
