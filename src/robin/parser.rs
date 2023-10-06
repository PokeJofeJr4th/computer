use std::{fmt::Display, iter::Peekable};

use super::types::{
    AssignOp, BlockType, Expression, Keyword, Statement, Token, TopLevelSyntax, UnaryOp,
};

#[derive(Debug)]
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

pub fn parse(src: Vec<Token>) -> Result<Vec<TopLevelSyntax>, ParseError> {
    let mut toks = src.into_iter().peekable();
    let mut top_level = Vec::new();
    while toks.peek().is_some() {
        top_level.extend(inner_parse_top_level(&mut toks)?);
    }
    Ok(top_level)
}

#[allow(clippy::too_many_lines)]
fn inner_parse_top_level<I: Iterator<Item = Token>>(
    src: &mut Peekable<I>,
) -> Result<Vec<TopLevelSyntax>, ParseError> {
    match src.next() {
        Some(Token::Keyword(Keyword::Fn)) => {
            let fn_name = match src.next() {
                Some(Token::Ident(i)) => i,
                Some(other) => {
                    return Err(ParseError::UnexpectedTokenExpectedStr(
                        other,
                        "function identifier".to_string(),
                    ))
                }
                None => return Err(ParseError::UnexpectedEOF),
            };
            match src.next() {
                Some(Token::LParen) => {}
                Some(other) => {
                    return Err(ParseError::UnexpectedTokenExpected(
                        other,
                        vec![Token::LParen],
                    ))
                }
                None => return Err(ParseError::UnexpectedEOF),
            };
            let args = match src.next() {
                Some(Token::RParen) => Vec::new(),
                Some(Token::Ident(arg)) => {
                    let mut args = vec![arg];
                    loop {
                        match src.next() {
                            Some(Token::RParen) => break,
                            Some(Token::Comma) => {}
                            Some(other) => {
                                return Err(ParseError::UnexpectedTokenExpected(
                                    other,
                                    vec![Token::RParen, Token::Comma],
                                ))
                            }
                            None => return Err(ParseError::UnexpectedEOF),
                        }
                        match src.next() {
                            Some(Token::Ident(arg)) => args.push(arg),
                            Some(other) => {
                                return Err(ParseError::UnexpectedTokenExpectedStr(
                                    other,
                                    "identifier".to_string(),
                                ))
                            }
                            None => return Err(ParseError::UnexpectedEOF),
                        }
                    }
                    args
                }
                Some(other) => {
                    return Err(ParseError::UnexpectedTokenExpected(
                        other,
                        vec![Token::RParen, Token::Ident("<identifier>".into())],
                    ))
                }
                None => return Err(ParseError::UnexpectedEOF),
            };
            let body = inner_parse_block(src)?;
            Ok(vec![TopLevelSyntax::Function(fn_name, args, body)])
        }
        Some(Token::Keyword(Keyword::Const)) => {
            let id = match src.next() {
                Some(Token::Ident(id)) => id,
                Some(other) => {
                    return Err(ParseError::UnexpectedTokenExpected(
                        other,
                        vec![Token::Ident("identifier".into())],
                    ))
                }
                None => return Err(ParseError::UnexpectedEOF),
            };
            match src.next() {
                Some(Token::Eq) => {}
                Some(other) => {
                    return Err(ParseError::UnexpectedTokenExpected(other, vec![Token::Eq]))
                }
                None => return Err(ParseError::UnexpectedEOF),
            }
            match src.next() {
                Some(Token::String(string)) => Ok(vec![TopLevelSyntax::Constant(
                    id,
                    Expression::String(string),
                )]),
                Some(Token::Int(int)) => {
                    Ok(vec![TopLevelSyntax::Constant(id, Expression::Int(int))])
                }
                Some(other) => Err(ParseError::UnexpectedTokenExpected(
                    other,
                    vec![Token::String("string literal".into()), Token::Int(u16::MAX)],
                )),
                None => Err(ParseError::UnexpectedEOF),
            }
        }
        Some(Token::SemiColon) => Ok(Vec::new()),
        Some(other) => Err(ParseError::UnexpectedTokenExpected(
            other,
            vec![Token::Keyword(Keyword::Fn), Token::Keyword(Keyword::Const)],
        )),
        None => Err(ParseError::UnexpectedEOF),
    }
}

fn inner_parse_block<I: Iterator<Item = Token>>(
    src: &mut Peekable<I>,
) -> Result<Vec<Statement>, ParseError> {
    match src.next() {
        Some(Token::LSquirrely) => {}
        Some(other) => {
            return Err(ParseError::UnexpectedTokenExpectedStr(
                other,
                "block".into(),
            ))
        }
        None => return Err(ParseError::UnexpectedEOF),
    }
    let mut body = Vec::new();
    while src.peek() != Some(&Token::RSquirrely) {
        body.push(inner_parse_statement(src)?);
        match src.next() {
            Some(Token::SemiColon) => {}
            Some(tok) => {
                return Err(ParseError::UnexpectedTokenExpected(
                    tok,
                    vec![Token::SemiColon],
                ))
            }
            None => return Err(ParseError::UnexpectedEOF),
        }
    }
    src.next();
    Ok(body)
}

fn inner_parse_statement<I: Iterator<Item = Token>>(
    src: &mut Peekable<I>,
) -> Result<Statement, ParseError> {
    match src.next() {
        Some(Token::Ident(ident)) => match src.next() {
            Some(tok) if AssignOp::try_from(tok.clone()).is_ok() => Ok(Statement::Assignment(
                ident,
                AssignOp::try_from(tok).unwrap(),
                inner_parse_expr(src)?,
            )),
            Some(Token::LParen) => {
                if src.peek() == Some(&Token::RParen) {
                    src.next();
                    return Ok(Statement::FunctionCall(ident, Vec::new()));
                }
                let mut args = vec![inner_parse_expr_greedy(src, 4)?];
                loop {
                    match src.next() {
                        Some(Token::Comma) => {
                            args.push(inner_parse_expr_greedy(src, 4)?);
                        }
                        Some(Token::RParen) => break,
                        Some(other) => {
                            return Err(ParseError::UnexpectedTokenExpected(
                                other,
                                vec![Token::Comma, Token::RParen],
                            ))
                        }
                        None => return Err(ParseError::UnexpectedEOF),
                    }
                }
                Ok(Statement::FunctionCall(ident, args))
            }
            Some(other) => Err(ParseError::UnexpectedTokenExpected(
                other,
                vec![Token::LParen, Token::Eq],
            )),
            _ => Err(ParseError::UnexpectedEOF),
        },
        Some(Token::Keyword(kw)) if BlockType::try_from(kw).is_ok() => {
            let cond = inner_parse_expr_greedy(src, 0)?;
            let body = inner_parse_block(src)?;
            let block_type = BlockType::try_from(kw).unwrap();
            Ok(Statement::Block(block_type, cond, body))
        }
        Some(other) => Err(ParseError::UnexpectedTokenExpectedStr(
            other,
            "statement".to_string(),
        )),
        None => Err(ParseError::UnexpectedEOF),
    }
}

fn inner_parse_expr_greedy<I: Iterator<Item = Token>>(
    src: &mut Peekable<I>,
    priority: u8,
) -> Result<Expression, ParseError> {
    if priority == 5 {
        return inner_parse_expr(src);
    }
    let mut start = inner_parse_expr_greedy(src, priority + 1)?;
    loop {
        match src.peek() {
            Some(Token::And | Token::Or | Token::Xor) if priority == 0 => {
                start = Expression::BinaryOp(
                    Box::new(start),
                    src.next().unwrap().try_into().unwrap(),
                    Box::new(inner_parse_expr_greedy(src, priority + 1)?),
                );
            }
            Some(
                Token::Eqeq | Token::BangEq | Token::Lt | Token::LtEq | Token::Gt | Token::GtEq,
            ) if priority == 1 => {
                start = Expression::BinaryOp(
                    Box::new(start),
                    src.next().unwrap().try_into().unwrap(),
                    Box::new(inner_parse_expr_greedy(src, priority + 1)?),
                );
            }
            Some(Token::Tack | Token::Plus) if priority == 2 => {}
            Some(Token::Star) if priority == 3 => {}
            Some(Token::BitAnd | Token::BitOr | Token::BitXor | Token::Shl | Token::Shr)
                if priority == 4 => {}
            Some(_) | None => break,
        }
    }
    Ok(start)
}

fn inner_parse_expr<I: Iterator<Item = Token>>(
    src: &mut Peekable<I>,
) -> Result<Expression, ParseError> {
    match src.next() {
        Some(tok) if UnaryOp::try_from(tok.clone()).is_ok() => Ok(Expression::UnaryOp(
            UnaryOp::try_from(tok).unwrap(),
            Box::new(inner_parse_expr(src)?),
        )),
        Some(Token::LParen) => {
            let inner = inner_parse_expr_greedy(src, 0)?;
            match src.next() {
                Some(Token::RParen) => Ok(inner),
                Some(other) => Err(ParseError::UnexpectedTokenExpected(
                    other,
                    vec![Token::RParen],
                )),
                None => Err(ParseError::UnexpectedEOF),
            }
        }
        Some(Token::Ident(ident)) => Ok(Expression::Ident(ident)),
        Some(Token::Int(i)) => Ok(Expression::Int(i)),
        Some(other) => Err(ParseError::UnexpectedTokenExpectedStr(
            other,
            "expression".to_string(),
        )),
        None => Err(ParseError::UnexpectedEOF),
    }
}
