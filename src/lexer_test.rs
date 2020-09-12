#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use crate::token;

   #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");

        let tests = [
            token::ASSIGN,
            token::PLUS,
            token::LPAREN,
            token::RPAREN,
            token::LBRACE,
            token::RBRACE,
            token::COMMA,
            token::SEMICOLON
        ];

        let lexer = new(input);

        for &test in tests.iter() {
            assert_eq!(lexer.next_token(), test);
        }
    }
}
