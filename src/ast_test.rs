#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::token;

    #[test]
    fn test_program_to_string() {
        let program = Program{
            statements: vec![
                Statements::Let(
                    LetStatement{
                        token: token::new(token::LET, "let".to_string()),
                        name: Identifier{
                            token: token::new(token::IDENT, "myVar".to_string()),
                            value: "myVar".to_string()
                        },
                        value: Expressions::Identifier(
                            Identifier{
                                token: token::new(token::IDENT, "anotherVar".to_string()),
                                value: "anotherVar".to_string()
                            }
                        )
                    }
                )
            ]
        };

        assert_eq!(program.to_string(), "let myVar = anotherVar;".to_string());
    }
}
