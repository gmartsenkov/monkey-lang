#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use crate::token;

   #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");

        let tests = [
            token::new(token::ASSIGN, String::from("=")),
            token::new(token::PLUS, String::from("+")),
            token::new(token::LPAREN, String::from("(")),
            token::new(token::RPAREN, String::from(")")),
            token::new(token::LBRACE, String::from("{")),
            token::new(token::RBRACE, String::from("}")),
            token::new(token::COMMA, String::from(",")),
            token::new(token::SEMICOLON, String::from(";"))
        ];

        let mut lexer = new(input);

        for test in tests.iter() {
            let token = lexer.next_token();
            assert_eq!(token.token_type, test.token_type);
        }
    }
}
