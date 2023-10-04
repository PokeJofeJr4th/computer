use std::str::FromStr;
use strum::EnumString;

use self::instruction::Value;

mod instruction;
mod syntax;

#[derive(EnumString, Debug, Clone, Copy, PartialEq, Eq)]
#[strum(ascii_case_insensitive)]
pub enum Keyword {
    Mov,
    Swp,
    Jmp,
    Jez,
    Jnz,
    Add,
    Sub,
    Mul,
    Jeq,
    Ceq,
    Jne,
    Cne,
    Jlt,
    Clt,
    Jle,
    Cle,
    Jgt,
    Cgt,
    Jge,
    Cge,
    Not,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Yield,
    Deref,
    Movptr,
}

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    Literal(Value),
    Address(Value),
    Label(String),
    SemiColon,
}

fn lex(src: &str) -> Option<Vec<Token>> {
    src.split_whitespace()
        .map(|str| {
            let (str, is_semicolon) = str
                .strip_suffix(';')
                .map_or((str, false), |str| (str, true));
            str.strip_prefix(':')
                .map_or_else(
                    || {
                        str.strip_prefix('&').map_or_else(
                            || {
                                str.strip_prefix('r').map_or_else(
                                    || {
                                        str.strip_prefix('#').map_or_else(
                                            || Keyword::from_str(str).ok().map(Token::Keyword),
                                            |str| {
                                                u16::from_str_radix(str, 16)
                                                    .map(Value::Given)
                                                    .map(Token::Literal)
                                                    .map_or_else(
                                                        |_| {
                                                            Some(Token::Literal(Value::Label(
                                                                String::from(str),
                                                            )))
                                                        },
                                                        Some,
                                                    )
                                            },
                                        )
                                    },
                                    |str| {
                                        u16::from_str_radix(str, 16)
                                            .map(Value::Given)
                                            .map(Token::Address)
                                            .ok()
                                    },
                                )
                            },
                            |str| {
                                u16::from_str_radix(str, 16)
                                    .map(Value::Given)
                                    .map(Token::Address)
                                    .map_or_else(
                                        |_| Some(Token::Address(Value::Label(String::from(str)))),
                                        Some,
                                    )
                            },
                        )
                    },
                    |str| Some(Token::Label(String::from(str))),
                )
                .map(|tok| {
                    if is_semicolon {
                        vec![tok, Token::SemiColon]
                    } else {
                        vec![tok]
                    }
                })
        })
        .collect::<Option<Vec<Vec<_>>>>()
        .map(Vec::into_iter)
        .map(Iterator::flatten)
        .map(Iterator::collect)
}

#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn compile_asm(src: &str) -> Option<Vec<u16>> {
    let toks = lex(src)?;
    println!("{toks:?}");
    syntax::interpret(&toks)
}
