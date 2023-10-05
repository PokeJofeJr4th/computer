use std::rc::Rc;

use strum::EnumString;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Ident(Rc<str>),
    String(Rc<str>),
    Int(u16),
    Keyword(Keyword),
    Eq,
    Plus,
    PlusEq,
    Tack,
    TackEq,
    Star,
    StarEq,
    Eqeq,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Bang,
    And,
    AndEq,
    BitAnd,
    Or,
    OrEq,
    BitOr,
    Xor,
    XorEq,
    BitXor,
    Shl,
    ShlEq,
    Shr,
    ShrEq,
    Comma,
    SemiColon,
    LSquirrely,
    RSquirrely,
    LParen,
    RParen,
}

#[derive(EnumString, Debug, PartialEq, Eq, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Keyword {
    Const,
    Fn,
    If,
    Int,
    While,
}

#[derive(Debug, Clone)]
pub enum TopLevelSyntax {
    Function(Rc<str>, Vec<Rc<str>>, Vec<Statement>),
    Constant(Rc<str>, Expression),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Declaration(Rc<str>, Option<Expression>),
    Assignment(Rc<str>, Expression),
    FunctionCall(Rc<str>, Vec<Expression>),
    While(Expression, Vec<Statement>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Ident(Rc<str>),
    Int(u16),
    BinaryOp(Box<Expression>, BinaryOperation, Box<Expression>),
    Not(Box<Expression>),
    Deref(Box<Expression>),
    FunctionCall(Rc<str>, Vec<Expression>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    BitAnd,
    Or,
    BitOr,
    Xor,
    BitXor,
    Shl,
    Shr,
}

impl TryFrom<Token> for BinaryOperation {
    type Error = Token;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Self::Add),
            Token::Tack => Ok(Self::Sub),
            Token::Star => Ok(Self::Mul),
            Token::Eqeq => Ok(Self::Eq),
            Token::BangEq => Ok(Self::Ne),
            Token::Lt => Ok(Self::Lt),
            Token::LtEq => Ok(Self::Le),
            Token::Gt => Ok(Self::Gt),
            Token::GtEq => Ok(Self::Ge),
            Token::And => Ok(Self::And),
            Token::BitAnd => Ok(Self::BitAnd),
            Token::Or => Ok(Self::Or),
            Token::BitOr => Ok(Self::BitOr),
            Token::Xor => Ok(Self::Xor),
            Token::BitXor => Ok(Self::BitXor),
            Token::Shl => Ok(Self::Shl),
            Token::Shr => Ok(Self::Shr),
            value => Err(value),
        }
    }
}

pub enum AssignOp {
    Eq,
    Add,
    Sub,
    Mul,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

impl TryFrom<Token> for AssignOp {
    type Error = Token;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Eq => Ok(Self::Eq),
            Token::PlusEq => Ok(Self::Add),
            Token::TackEq => Ok(Self::Sub),
            Token::StarEq => Ok(Self::Mul),
            Token::AndEq => Ok(Self::And),
            Token::OrEq => Ok(Self::Or),
            Token::XorEq => Ok(Self::Xor),
            Token::ShlEq => Ok(Self::Shl),
            Token::ShrEq => Ok(Self::Shr),
            value => Err(value),
        }
    }
}
