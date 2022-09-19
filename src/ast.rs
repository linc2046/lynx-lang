use crate::token::TokenType;

// semantic type alias
pub type Operator = TokenType;
pub type PrefixExpression = Box<Expression>;
pub type InfixExpression = Box<Expression>;
pub type HashKey = Expression;
pub type HashValue = Expression;
pub type FnName = Box<Expression>;
pub type FnParameter = Vec<Expression>;
pub type FnBody = Statement;
pub type IfCondition = Box<Expression>;
pub type WhileCondition = Box<Expression>;

#[derive(Debug, Clone)]
pub enum AstNode {
    // let a = 123;
    // let b = fn(foo) { return foo + 1; };
    // return b(a);
    Program(Vec<Statement>),
    // // let a = 123;
    // Stmt(Statement),
    // // 1 + 2;
    // // [1, 2, 3]
    // Expr(Expression),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    // foo
    Identifier(String),

    // 1234
    Integer(usize),

    // true | false
    Boolean(bool),

    // "string"
    String(String),

    // [<expression>, <expression>, ...];
    Array(Vec<Expression>),

    // { <expression>: <expression>, ... }
    Hash(Vec<(HashKey, HashValue)>),

    // <operator> <expression>
    Prefix(Operator, PrefixExpression),

    // <expression> <operator> <expression>
    Infix(InfixExpression, Operator, InfixExpression),

    // if (<expression>) { <statement> | <expression> } else { <statement> | <expression> }
    If(IfCondition, Statement, Option<Statement>),

    // while (<expression>) { <block statement> }
    While(WhileCondition, Statement),

    // break while loop
    Break,

    // fn <identifier>(<parameter one>, <parameter two>, ...) {  <block statement>  };
    // let foo = fn(bar) => { puts(bar); }
    Fn(FnName, FnParameter, FnBody),

    // <identifier>(<expression>, <expression>, ...)
    // a(1 + 1, 2, b(1))
    FnCall(FnName, FnParameter),

    // (token_position, token_type)
    NON_PARSED_EXPR((usize, TokenType)),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    // let <identifier> = <expression>;
    Let(Box<Expression>, Box<Expression>),

    // return <expression>
    Return(Box<Expression>),

    // <expression>
    Expr(Box<Expression>),

    // { <statement>, <statement>, ... }
    BlockStatement(Vec<Statement>),
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,   // ==
    Compare,  // < or >
    Addition, // + or  -
    Multiply, // * or /
    Prefix,   // -x or !x
    Group,    // (1 + 2) / 3 or foo(x)
    Index,    // array[index]
}

impl Precedence {
    pub fn get(token: &TokenType) -> Precedence {
        match token {
            TokenType::ADD | TokenType::MINUS => Precedence::Addition,
            TokenType::MULTIPLY | TokenType::DIVIDE => Precedence::Multiply,
            TokenType::BANG => Precedence::Prefix, // | TokenType::MINUS
            TokenType::LEFT_PAREN => Precedence::Group,
            TokenType::LEFT_BRACE => Precedence::Index,
            TokenType::EQUAL_EQUAL | TokenType::LESS_EQUAL | TokenType::GREATER_EQUAL => Precedence::Equals,
            TokenType::LESS | TokenType::GREATER => Precedence::Compare,
            _ => Precedence::Lowest,
        }
    }
}
