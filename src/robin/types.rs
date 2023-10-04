use std::rc::Rc;

use strum::EnumString;

#[derive(Debug)]
pub enum Token {
    Ident(Rc<str>),
    String(Rc<str>),
    Int(u16),
    Keyword(Keyword),
    Eq,
    Plus,
    Tack,
    Star,
    Eqeq,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Bang,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Comma,
    SemiColon,
    LSquirrely,
    RSquirrely,
    LParen,
    RParen,
}

#[derive(EnumString, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Keyword {
    Int,
    Fn,
}

pub enum Statement {
    Declaration(Rc<str>, Option<Expression>),
    Assignment(Rc<str>, Expression),
    Function(Rc<str>, Vec<Rc<str>>, Vec<Statement>),
    FunctionCall(Rc<str>, Vec<Expression>),
}

pub enum Expression {
    Ident(Rc<str>),
    BinaryOp(Box<Expression>, BinaryOperation, Box<Expression>),
    Not(Box<Expression>),
    Deref(Box<Expression>),
    FunctionCall(Rc<str>, Vec<Expression>),
}

pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}
