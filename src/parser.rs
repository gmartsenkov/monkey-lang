use crate::{ast, lexer, token};

pub struct Parser<'a> {
    lexer: &'a mut lexer::Lexer,
    current_token: token::Token,
    peek_token: token::Token,
    errors: Vec<String>,
}

pub fn new(lexer: &mut lexer::Lexer) -> Parser {
    let mut parser = Parser {
        lexer: lexer,
        current_token: token::new(token::NULL, "".to_string()),
        peek_token: token::new(token::NULL, "".to_string()),
        errors: vec![],
    };

    parser.next_token();
    parser.next_token();

    parser
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
            _ => None,
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
