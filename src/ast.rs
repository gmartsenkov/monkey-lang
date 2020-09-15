use crate::token;

pub enum Statements {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement)
}

pub enum Expressions {
    Idedntifier(Identifier)
}

pub struct Program {
    pub statements : Vec<Statements>
}

pub struct Identifier {
    pub token : token::Token,
    pub value : String
}

pub struct LetStatement {
    pub token : token::Token,
    pub name : Identifier,
    // pub value : Expressions
}

pub struct ReturnStatement {
    pub token : token::Token,
    // pub return_value : Expressions
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

impl Statements {
    pub fn token_literal(&self) -> &str {
        match self {
            Statements::LetStatement(i) => i.token.literal.as_str(),
            Statements::ReturnStatement(i) => i.token.literal.as_str()
        }
    }
}

impl Program {
    fn token_literal(&self) -> &str {
        if self.statements.len() > 1 {
            return self.statements[0].token_literal();
        }
        ""
    }
}
