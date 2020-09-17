#[cfg(test)]
mod tests {
    use crate::parser::*;
    use crate::{ast, lexer};

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

        assert_eq!(program.statements.len(), 6);

        let expected = [
            ("let", "x"),
            ("5", "5"),
            ("let", "y"),
            ("10", "10"),
            ("let", "foobar"),
            ("838383", "838383"),
        ];

        for (index, &val) in expected.iter().enumerate() {
            let statement = &program.statements[index];

            assert_eq!(statement.token_literal(), val.0);

            match statement {
                ast::Statements::Let(s) => {
                    assert_eq!(s.name.value, val.1);
                    assert_eq!(s.name.token_literal(), val.1);
                }
                ast::Statements::Expression(e) => assert_eq!(e.token.literal, val.1),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_return_statement() {
        let input = r#"
          return 5;
          return 10;
          return 993322;
        "#;

        let mut lexer = lexer::new(input.to_string());
        let mut parser = new(&mut lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);

        for statement in program.statements.iter() {
            match statement {
                ast::Statements::Return(_) => {}
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn text_identifier_expression() {
        let input = "foobar;";
        let mut lexer = lexer::new(input.to_string());
        let mut parser = new(&mut lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        for statement in program.statements.iter() {
            match statement {
                ast::Statements::Expression(i) => match &i.expression {
                    ast::Expressions::Identifier(e) => {
                        assert_eq!(e.value, "foobar");
                        assert_eq!(e.token_literal(), "foobar");
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn text_integer_literal_expression() {
        let input = "5;";
        let mut lexer = lexer::new(input.to_string());
        let mut parser = new(&mut lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        for statement in program.statements.iter() {
            match statement {
                ast::Statements::Expression(i) => match &i.expression {
                    ast::Expressions::IntegerLiteral(e) => {
                        assert_eq!(e.value, 5);
                        assert_eq!(e.token_literal(), "5");
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_prefix_operator() {
        let tests = [("!5;", "!", 5), ("-15;", "-", 15)];

        for &test in tests.iter() {
            let input = test.0;
            let mut lexer = lexer::new(input.to_string());
            let mut parser = new(&mut lexer);

            let program = parser.parse_program();

            assert_eq!(program.statements.len(), 1);

            for statement in program.statements.iter() {
                match statement {
                    ast::Statements::Expression(i) => match &i.expression {
                        ast::Expressions::Prefix(e) => {
                            assert_eq!(e.operator, test.1);
                            match &*e.right {
                                ast::Expressions::IntegerLiteral(i) => {
                                    assert_eq!(i.value, test.2);
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn test_inflix_parsing() {
        let tests = [
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for &test in tests.iter() {
            let input = test.0;
            let mut lexer = lexer::new(input.to_string());
            let mut parser = new(&mut lexer);

            let program = parser.parse_program();

            assert_eq!(program.statements.len(), 1);

            for statement in program.statements.iter() {
                match statement {
                    ast::Statements::Expression(e) => match &e.expression {
                        ast::Expressions::Infix(i) => {
                            match &*i.left {
                                ast::Expressions::IntegerLiteral(l) => assert_eq!(l.value, test.1),
                                _ => unreachable!(),
                            }
                            assert_eq!(i.operator, test.2);
                            match &*i.right {
                                ast::Expressions::IntegerLiteral(l) => assert_eq!(l.value, test.1),
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn test_precedence_parsing() {
        let tests = [
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];

        for &test in tests.iter() {
            let input = test.0;
            let mut lexer = lexer::new(input.to_string());
            let mut parser = new(&mut lexer);

            let program = parser.parse_program();

            assert_eq!(program.to_string(), test.1);
        }
    }
}
