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
            let identifier = statement.expression().identifier();

            assert_eq!(identifier.value, "foobar");
            assert_eq!(identifier.token_literal(), "foobar");
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
            let integer_literal = statement.expression().integer_literal();

            assert_eq!(integer_literal.value, 5);
            assert_eq!(integer_literal.token_literal(), "5");
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
                let prefix = statement.expression().prefix();

                assert_eq!(prefix.operator, test.1);
                assert_eq!(prefix.right.integer_literal().value, test.2);
            }
        }
    }

    #[test]
    fn test_prefix_operator_with_bool() {
        let tests = [("!true;", "!", true), ("!false;", "!", false)];

        for &test in tests.iter() {
            let input = test.0;
            let mut lexer = lexer::new(input.to_string());
            let mut parser = new(&mut lexer);

            let program = parser.parse_program();

            assert_eq!(program.statements.len(), 1);

            for statement in program.statements.iter() {
                let prefix = statement.expression().prefix();

                assert_eq!(prefix.operator, test.1);
                assert_eq!(prefix.right.boolean().value, test.2);
            }
        }
    }
    #[test]
    fn test_inflix_parsing() {
        let tests = [
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 15;", 5, "/", 15),
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
                let infix = statement.expression().infix();

                assert_eq!(infix.left.integer_literal().value, test.1);
                assert_eq!(infix.operator, test.2);
                assert_eq!(infix.right.integer_literal().value, test.3);
            }
        }
    }

    #[test]
    fn test_inflix_boolean_parsing() {
        let tests = [
            ("true == true", true, "==", true),
            ("true != false", true, "!=", false),
            ("false == false", false, "==", false),
        ];

        for &test in tests.iter() {
            let input = test.0;
            let mut lexer = lexer::new(input.to_string());
            let mut parser = new(&mut lexer);

            let program = parser.parse_program();

            assert_eq!(program.statements.len(), 1);

            for statement in program.statements.iter() {
                let infix = statement.expression().infix();

                assert_eq!(infix.left.boolean().value, test.1);
                assert_eq!(infix.operator, test.2);
                assert_eq!(infix.right.boolean().value, test.3);
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
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];

        for &test in tests.iter() {
            let input = test.0;
            let mut lexer = lexer::new(input.to_string());
            let mut parser = new(&mut lexer);

            let program = parser.parse_program();

            assert_eq!(program.to_string(), test.1);
        }
    }

    #[test]
    fn test_boolean_parsing() {
        let input = r#"
          true;
          false;
          let foobar = true;
          let barfoo = false;
        "#;

        let mut lexer = lexer::new(input.to_string());
        let mut parser = new(&mut lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 6);

        assert_eq!(program.statements[0].expression().boolean().value, true);
        assert_eq!(program.statements[1].expression().boolean().value, false);
        assert_eq!(program.statements[2].let_statement().name.value, "foobar");
        assert_eq!(program.statements[3].expression().boolean().value, true);
        assert_eq!(program.statements[4].let_statement().name.value, "barfoo");
        assert_eq!(program.statements[5].expression().boolean().value, false);
    }

    #[test]
    fn test_if_expression_parsing() {
        let input = "if (x < y) { x } ";

        let mut lexer = lexer::new(input.to_string());
        let mut parser = new(&mut lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let statement = program.statements[0].expression().if_statement();
        assert_eq!(statement.condition.infix().left.identifier().value, "x");
        assert_eq!(statement.condition.infix().operator, "<");
        assert_eq!(statement.condition.infix().right.identifier().value, "y");

        assert_eq!(statement.alternative.is_none(), true);

        let consequence = &statement.consequence;
        assert_eq!(consequence.statements.len(), 1);

        let statement = consequence.statements[0].expression().identifier();
        assert_eq!(statement.token_literal(), "x");
    }

    #[test]
    fn test_if_else_expression_parsing() {
        let input = "if (x < y) { x } else { y }";

        let mut lexer = lexer::new(input.to_string());
        let mut parser = new(&mut lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let statement = program.statements[0].expression().if_statement();
        assert_eq!(statement.condition.infix().left.identifier().value, "x");
        assert_eq!(statement.condition.infix().operator, "<");
        assert_eq!(statement.condition.infix().right.identifier().value, "y");

        assert_eq!(statement.alternative.is_some(), true);

        let alternative = statement.alternative.as_ref().unwrap();
        assert_eq!(alternative.statements.len(), 1);
        assert_eq!(alternative.statements[0].token_literal(), "y");

        let consequence = &statement.consequence;
        assert_eq!(consequence.statements.len(), 1);
        assert_eq!(consequence.statements[0].token_literal(), "x");

    }
}
