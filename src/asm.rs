use std::str::FromStr;
use strum::EnumString;

mod instruction;
mod syntax;

#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
enum Token {
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
    Literal(u16),
    Register(u16),
    Address(u16),
    AddressName(String),
    Label(String),
    SemiColon,
}

fn lex(src: &str) -> Option<Vec<Token>> {
    src.split_whitespace()
        .map(|str| {
            str.strip_prefix(':').map_or_else(
                || {
                    str.strip_prefix('&').map_or_else(
                        || {
                            str.strip_prefix('r').map_or_else(
                                || {
                                    str.strip_prefix('#').map_or_else(
                                        || Token::from_str(str).ok(),
                                        |str| u16::from_str_radix(str, 16).map(Token::Literal).ok(),
                                    )
                                },
                                |str| u16::from_str_radix(str, 16).map(Token::Register).ok(),
                            )
                        },
                        |str| {
                            u16::from_str_radix(str, 16)
                                .map(Token::Address)
                                .map_or_else(|_| Some(Token::AddressName(String::from(str))), Some)
                        },
                    )
                },
                |str| Some(Token::Label(String::from(str))),
            )
        })
        .collect()
}

#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn compile_asm(src: &str) -> Option<Vec<u16>> {
    syntax::interpret(&lex(src)?)
}
