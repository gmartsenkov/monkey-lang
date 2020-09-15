#[cfg(test)]
mod tests {
    use crate::parser::*;
    use crate::{lexer, ast};

    #[test]
    fn test_let_statement() {
        let input = r#"
          let x = 5;
          let y = 10;
          let foobar = 838383;
        "#;
        let mut lexer = lexer::new(input.to_string());
        let mut parser = new(&mut lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);

        let expected = ["x", "y", "foobar"];

        for (index, &val) in expected.iter().enumerate() {
            let statement = &program.statements[index];

            assert_eq!(statement.token_literal(), "let");

            match statement {
                ast::Statements::LetStatement(s) => {
                    s.name.value == val;
                    s.name.token_literal() == val;
                }
                _ => unreachable!()
            }
        }
    }
}
