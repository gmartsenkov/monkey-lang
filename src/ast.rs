use crate::token;

pub enum Statements {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

pub enum Expressions {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Boolean(Boolean),
    If(IfStatement),
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

pub struct Boolean {
    pub token: token::Token,
    pub value: bool,
}

pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Expressions,
}

pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Box<Expressions>,
}

pub struct InfixExpression {
    pub token: token::Token,
    pub left: Box<Expressions>,
    pub operator: String,
    pub right: Box<Expressions>,
}

pub struct IfStatement {
    pub token: token::Token,
    pub condition: Box<Expressions>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Statements>,
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

impl PrefixExpression {
    pub fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl IfStatement {
    fn to_string(&self) -> String {
        let mut output = format!(
            "if {} {}",
            self.condition.to_string(),
            self.consequence.to_string()
        );

        if let Some(alternative) = &self.alternative {
            output += format!("else {}", alternative.to_string()).as_str();
        }
        output
    }
}

impl BlockStatement {
    fn to_string(&self) -> String {
        let mut output = String::new();

        for statement in &self.statements {
            output += statement.to_string().clone().as_str();
        }

        output
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

    pub fn expression(&self) -> &Expressions {
        match self {
            Statements::Expression(e) => &e.expression,
            _ => panic!("Not an expression."),
        }
    }

    pub fn let_statement(&self) -> &LetStatement {
        match self {
            Statements::Let(l) => &l,
            _ => panic!("Not a let statement."),
        }
    }
}

impl Expressions {
    fn to_string(&self) -> String {
        match &self {
            Expressions::Identifier(v) => v.value.clone(),
            Expressions::IntegerLiteral(v) => v.value.to_string().clone(),
            Expressions::Boolean(v) => v.value.to_string().clone(),
            Expressions::Prefix(v) => format!("({}{})", v.operator, v.right.to_string()),
            Expressions::Infix(v) => format!(
                "({} {} {})",
                v.left.to_string(),
                v.operator,
                v.right.to_string()
            ),
            Expressions::If(i) => i.to_string(),
        }
    }

    pub fn identifier(&self) -> &Identifier {
        match self {
            Expressions::Identifier(i) => &i,
            _ => panic!("Not an identifier expression."),
        }
    }

    pub fn integer_literal(&self) -> &IntegerLiteral {
        match self {
            Expressions::IntegerLiteral(i) => &i,
            _ => panic!("Not an integer literal expression."),
        }
    }

    pub fn prefix(&self) -> &PrefixExpression {
        match self {
            Expressions::Prefix(p) => &p,
            _ => panic!("Not an prefix expression."),
        }
    }

    pub fn infix(&self) -> &InfixExpression {
        match self {
            Expressions::Infix(i) => &i,
            _ => panic!("Not an infix expression."),
        }
    }

    pub fn boolean(&self) -> &Boolean {
        match self {
            Expressions::Boolean(b) => &b,
            _ => panic!("Not an infix expression."),
        }
    }

    pub fn if_statement(&self) -> &IfStatement {
        match self {
            Expressions::If(i) => &i,
            _ => panic!("Not an if statement"),
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
