use crate::token;

pub enum Statements {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

pub enum Expressions {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
}

pub struct Program {
    pub statements: Vec<Statements>,
}

pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Expressions,
}

pub struct ReturnStatement {
    pub token: token::Token,
    // pub return_value : Expressions
}

pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Expressions,
}

impl LetStatement {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl Identifier {
    pub fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl IntegerLiteral {
    pub fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl Statements {
    pub fn token_literal(&self) -> String {
        match self {
            Statements::Let(i) => i.token.literal.clone(),
            Statements::Return(i) => i.token.literal.clone(),
            Statements::Expression(i) => i.expression.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Statements::Let(s) => format!(
                "{} {} = {};",
                s.token_literal(),
                s.name.token_literal(),
                s.value.to_string()
            ),
            Statements::Return(s) => format!("{};", s.token.literal),
            Statements::Expression(s) => format!("{}", s.expression.to_string()),
        }
    }
}

impl Expressions {
    fn to_string(&self) -> String {
        match &self {
            Expressions::Identifier(v) => v.value.clone(),
            Expressions::IntegerLiteral(v) => v.value.to_string().clone(),
        }
    }
}

impl Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 1 {
            return self.statements[0].token_literal();
        }
        "".to_string()
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        for statement in &self.statements {
            output.push_str(&statement.to_string().clone());
        }

        output
    }
}
