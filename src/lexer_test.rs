#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use crate::token;

   #[test]
    fn test_next_token() {
        let input = String::from(r#"
           let five = 5;
           let ten = 10;
           let add = fn(x, y) {
               x + y;
           };
           let result = add(five, ten);"#
        );

        let tests = [
            token::new(token::LET, "let".to_string()),
            token::new(token::IDENT, "five".to_string()),
            token::new(token::ASSIGN, "=".to_string()),
            token::new(token::INT, "5".to_string()),
            token::new(token::SEMICOLON, ";".to_string()),
            token::new(token::LET, "let".to_string()),
            token::new(token::IDENT, "ten".to_string()),
            token::new(token::ASSIGN, "=".to_string()),
            token::new(token::INT, "10".to_string()),
            token::new(token::SEMICOLON, ";".to_string()),
            token::new(token::LET, "let".to_string()),
            token::new(token::IDENT, "add".to_string()),
            token::new(token::ASSIGN, "=".to_string()),
            token::new(token::FUNCTION, "fn".to_string()),
            token::new(token::LPAREN, "(".to_string()),
            token::new(token::IDENT, "x".to_string()),
            token::new(token::COMMA, ",".to_string()),
            token::new(token::IDENT, "y".to_string()),
            token::new(token::RPAREN, ")".to_string()),
            token::new(token::LBRACE, "{".to_string()),
            token::new(token::IDENT, "x".to_string()),
            token::new(token::PLUS, "+".to_string()),
            token::new(token::IDENT, "y".to_string()),
            token::new(token::SEMICOLON, ";".to_string()),
            token::new(token::RBRACE, "}".to_string()),
            token::new(token::SEMICOLON, ";".to_string()),
            token::new(token::LET, "let".to_string()),
            token::new(token::IDENT, "result".to_string()),
            token::new(token::ASSIGN, "=".to_string()),
            token::new(token::IDENT, "add".to_string()),
            token::new(token::LPAREN, "(".to_string()),
            token::new(token::IDENT, "five".to_string()),
            token::new(token::COMMA, ",".to_string()),
            token::new(token::IDENT, "ten".to_string()),
            token::new(token::RPAREN, ")".to_string()),
            token::new(token::SEMICOLON, ";".to_string()),
            token::new(token::EOF, "".to_string())
        ];

        let mut lexer = new(input);

        for test in tests.iter() {
            let token = lexer.next_token();
            assert_eq!(token.token_type, test.token_type);
        }
    }
}
