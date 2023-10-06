use std::{rc::Rc, str::FromStr};
use strum::EnumString;

mod instruction;
mod syntax;

pub use instruction::{CmpOp, Instruction, Item, MathOp, Value};
pub use syntax::{interpret_syntax, Syntax};

#[derive(Debug)]
pub enum ASMError {
    TokenError,
    SyntaxError(Vec<Token>),
}

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
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Yield,
    Ptrread,
    Ptrwrite,
    Reserve,
}

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    Literal(Value),
    Address(Value),
    Label(Rc<str>),
    SemiColon,
}

pub fn string_literal(src: &str) -> Vec<Syntax> {
    let mut toks = Vec::with_capacity(src.len() + 1);
    for c in src.chars() {
        toks.push(Syntax::Literal(u16::try_from(c as u32).unwrap_or(0xFFFE)));
    }
    toks.push(Syntax::Literal(0));
    toks
}

fn lex(src: &str) -> Option<Vec<Token>> {
    // apply string literals
    let mut stringparts = src.split('"');
    let mut string = String::with_capacity(src.len());
    loop {
        let Some(code) = stringparts.next() else { break };
        string.push_str(code);
        let Some(lit) = stringparts.next() else { break };
        for char in lit.chars() {
            string.push('#');
            string.push_str(&format!(
                "{:X}",
                u16::try_from(char as u32).unwrap_or(0xFFFE)
            ));
            string.push(';');
            string.push(' ');
        }
        string.push_str("#0; ");
    }
    // get actual tokens
    string
        .split_whitespace()
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
                                                                Rc::from(str),
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
                                        |_| Some(Token::Address(Value::Label(Rc::from(str)))),
                                        Some,
                                    )
                            },
                        )
                    },
                    |str| Some(Token::Label(Rc::from(str))),
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

#[allow(clippy::module_name_repetitions)]
/// # Errors
/// if the asm syntax is bad
pub fn compile_asm(src: &str) -> Result<Vec<u16>, ASMError> {
    let toks = lex(src).ok_or(ASMError::TokenError)?;
    // println!("{toks:?}");
    syntax::interpret(&toks).map_err(ASMError::SyntaxError)
}
