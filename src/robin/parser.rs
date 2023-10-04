use std::{fmt::Display, iter::Peekable};

use super::types::{Expression, Statement, Token};

pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedTokenExpected(Token, Vec<Token>),
    UnexpectedTokenExpectedStr(Token, String),
    UnexpectedEOF,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEOF => write!(f, "Unexpected end of file"),
            Self::UnexpectedToken(tok) => write!(f, "Unexpected token {tok:?}"),
            Self::UnexpectedTokenExpected(tok, exp) => {
                write!(f, "Unexpected token {tok:?}; Expected one of {exp:?}")
            }
            Self::UnexpectedTokenExpectedStr(tok, exp) => {
                write!(f, "Unexpected token {tok:?}; Expected {exp}")
            }
        }
    }
}

pub fn parse(src: Vec<Token>) -> Result<Vec<Statement>, ParseError> {
    let mut toks = src.into_iter().peekable();
    inner_parse_statement(&mut toks)
}

fn inner_parse_statement<I: Iterator<Item = Token>>(
    src: &mut Peekable<I>,
) -> Result<Vec<Statement>, ParseError> {
    match src.next() {
        Some(Token::Ident(ident)) => match src.next() {
            Some(Token::Eq) => {
                let expr = inner_parse_expr(src)?;
                match src.next() {
                    Some(Token::SemiColon) => Ok(vec![Statement::Assignment(ident, expr)]),
                    Some(other) => Err(ParseError::UnexpectedToken(other)),
                    None => Err(ParseError::UnexpectedEOF),
                }
            }
            Some(other) => Err(ParseError::UnexpectedToken(other)),
            _ => Err(ParseError::UnexpectedEOF),
        },
        Some(other) => Err(ParseError::UnexpectedToken(other)),
        None => Err(ParseError::UnexpectedEOF),
    }
}

fn inner_parse_expr<I: Iterator<Item = Token>>(
    src: &mut Peekable<I>,
) -> Result<Expression, ParseError> {
    match src.next() {
        Some(Token::Star) => Ok(Expression::Deref(Box::new(inner_parse_expr(src)?))),
        Some(Token::Ident(ident)) => Ok(Expression::Ident(ident)),
        Some(other) => Err(ParseError::UnexpectedToken(other)),
        None => Err(ParseError::UnexpectedEOF),
    }
}
