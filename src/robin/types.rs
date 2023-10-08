use std::rc::Rc;

use strum::{AsRefStr, EnumString};

use crate::asm::{CmpOp, MathOp};

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
    LSquare,
    RSquare,
}

#[derive(EnumString, Debug, PartialEq, Eq, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Keyword {
    Const,
    Global,
    Fn,
    If,
    Return,
    Var,
    While,
}

#[derive(Debug, Clone)]
pub enum TopLevelSyntax {
    Function(Rc<str>, Vec<Rc<str>>, Vec<Statement>),
    Constant(Rc<str>, Expression),
    Global(Rc<str>, Expression),
}

#[derive(Debug, Clone, Hash)]
pub enum Statement {
    Declaration(Rc<str>, Option<Expression>),
    Return(Option<Expression>),
    Assignment(Rc<str>, AssignOp, Expression),
    StarAssignment(Expression, Expression),
    FunctionCall(Rc<str>, Vec<Expression>),
    Block(BlockType, Expression, Vec<Statement>),
}

#[derive(Debug, Clone, Hash)]
pub enum Expression {
    Ident(Rc<str>),
    Array(Vec<u16>),
    String(Rc<str>),
    Int(u16),
    BinaryOp(Box<Expression>, BinaryOp, Box<Expression>),
    UnaryOp(UnaryOp, Box<Expression>),
    FunctionCall(Rc<str>, Vec<Expression>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum BlockType {
    While,
    If,
}

impl TryFrom<Keyword> for BlockType {
    type Error = Keyword;
    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            Keyword::While => Ok(Self::While),
            Keyword::If => Ok(Self::If),
            value => Err(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Not,
    Deref,
    Address,
}

impl TryFrom<Token> for UnaryOp {
    type Error = Token;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Bang => Ok(Self::Not),
            Token::Star => Ok(Self::Deref),
            Token::BitAnd => Ok(Self::Address),
            value => Err(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
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

impl TryFrom<Token> for BinaryOp {
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

impl TryFrom<BinaryOp> for crate::asm::CmpOp {
    type Error = BinaryOp;
    fn try_from(value: BinaryOp) -> Result<Self, Self::Error> {
        match value {
            BinaryOp::Eq => Ok(Self::Eq),
            BinaryOp::Ne => Ok(Self::Ne),
            BinaryOp::Gt => Ok(Self::Gt),
            BinaryOp::Ge => Ok(Self::Ge),
            BinaryOp::Lt => Ok(Self::Lt),
            BinaryOp::Le => Ok(Self::Le),
            value => Err(value),
        }
    }
}

impl From<CmpOp> for BinaryOp {
    fn from(value: CmpOp) -> Self {
        match value {
            CmpOp::Eq => Self::Eq,
            CmpOp::Ne => Self::Ne,
            CmpOp::Lt => Self::Lt,
            CmpOp::Le => Self::Le,
            CmpOp::Gt => Self::Gt,
            CmpOp::Ge => Self::Ge,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl TryFrom<AssignOp> for MathOp {
    type Error = AssignOp;
    fn try_from(value: AssignOp) -> Result<Self, Self::Error> {
        match value {
            AssignOp::Add => Ok(Self::Add),
            AssignOp::Sub => Ok(Self::Sub),
            AssignOp::Mul => Ok(Self::Mul),
            AssignOp::And => Ok(Self::And),
            AssignOp::Or => Ok(Self::Or),
            AssignOp::Xor => Ok(Self::Xor),
            AssignOp::Shl => Ok(Self::Shl),
            AssignOp::Shr => Ok(Self::Shr),
            AssignOp::Eq => Err(AssignOp::Eq),
        }
    }
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
