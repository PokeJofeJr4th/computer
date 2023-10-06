use std::{fmt::Display, iter::Peekable};

use super::types::{Keyword, Token};

#[derive(Debug)]
pub enum LexError {
    UnexpectedChar(char),
    UnexpectedEOF,
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedChar(c) => write!(f, "Unexpected character {c:?}"),
            Self::UnexpectedEOF => write!(f, "Unexpected end of file"),
        }
    }
}

pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let mut chars = src.chars().peekable();
    let mut tokens = Vec::new();
    while chars.peek().is_some() {
        lex_inner(&mut chars, &mut tokens)?;
    }
    Ok(tokens)
}

macro_rules! multi_character_pattern {
    ($chars:ident $just:expr; {$($char:expr => $eq:expr),*}) => {
        match $chars.peek() {
            $(Some($char) => {
                $chars.next();
                $eq
            })*
            _ => $just,
        }
    };
}

fn lex_inner<I: Iterator<Item = char>>(
    chars: &mut Peekable<I>,
    tokens: &mut Vec<Token>,
) -> Result<(), LexError> {
    tokens.push(match chars.next() {
        Some('=') => multi_character_pattern!(chars Token::Eq; {'=' => Token::Eqeq}),
        Some('<') => {
            multi_character_pattern!(chars Token::Lt; {'=' => Token::LtEq, '<' => Token::Shl})
        }
        Some('>') => {
            multi_character_pattern!(chars Token::Gt; {'=' => Token::GtEq, '>' => Token::Shr})
        }
        Some('!') => multi_character_pattern!(chars Token::Bang; {'=' => Token::BangEq}),
        Some('+') => multi_character_pattern!(chars Token::Plus; {'=' => Token::PlusEq}),
        Some('-') => multi_character_pattern!(chars Token::Tack; {'=' => Token::TackEq}),
        Some('&') => {
            multi_character_pattern!(chars Token::BitAnd; {'&' => Token::And, '=' => Token::AndEq})
        }
        Some('|') => {
            multi_character_pattern!(chars Token::BitOr; {'|' => Token::Or, '=' => Token::OrEq})
        }
        Some('^') => {
            multi_character_pattern!(chars Token::BitXor; {'^' => Token::Xor, '=' => Token::XorEq})
        }
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
        Some('"') => {
            let mut str_buf = String::new();
            for c in chars.by_ref() {
                if c == '"' {
                    break;
                }
                str_buf.push(c);
            }
            Token::String(str_buf.into())
        }
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
