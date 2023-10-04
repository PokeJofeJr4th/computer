use std::iter::Peekable;

use super::types::{Keyword, Token};

pub enum LexError {
    UnexpectedChar(char),
    UnexpectedEOF,
}

pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let mut chars = src.chars().peekable();
    let mut tokens = Vec::new();
    while chars.peek().is_some() {
        lex_inner(&mut chars, &mut tokens)?;
    }
    Ok(tokens)
}

fn lex_inner<I: Iterator<Item = char>>(
    chars: &mut Peekable<I>,
    tokens: &mut Vec<Token>,
) -> Result<(), LexError> {
    tokens.push(match chars.next() {
        Some('=') => Token::Eq,
        Some('+') => Token::Plus,
        Some('-') => Token::Tack,
        Some('*') => Token::Star,
        Some(',') => Token::Comma,
        Some(';') => Token::SemiColon,
        Some('{') => Token::LSquirrely,
        Some('}') => Token::RSquirrely,
        Some('(') => Token::LParen,
        Some(')') => Token::RParen,
        Some(first @ '0'..='9') => {
            let mut int_buf = String::from(first);
            while let Some(&next @ '0'..='9') = chars.peek() {
                chars.next();
                int_buf.push(next);
            }
            Token::Int(int_buf.parse().unwrap())
        }
        Some(first @ ('a'..='z' | 'A'..='Z' | '_')) => {
            let mut ident_buf = String::from(first);
            while let Some(&next @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_')) = chars.peek() {
                chars.next();
                ident_buf.push(next);
            }
            Keyword::try_from(ident_buf.as_ref())
                .map_or_else(|_| Token::Ident(ident_buf.into()), Token::Keyword)
        }
        Some('"') => todo!(),
        Some(other) => {
            if other.is_whitespace() {
                return Ok(());
            }
            return Err(LexError::UnexpectedChar(other));
        }
        None => return Err(LexError::UnexpectedEOF),
    });
    Ok(())
}
