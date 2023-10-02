use std::collections::BTreeMap;

use super::{
    instruction::{Instruction, Item},
    Keyword, Token,
};

pub enum Syntax {
    Label(String),
    Instruction(Instruction),
}

pub(super) fn interpret(src: &[Token]) -> Option<Vec<u16>> {
    // get the syntax
    let statements = interpret_tokens(src)?;
    // first pass to get location of all the labels
    let mut byte_location: usize = 0x8000;
    let mut labels = BTreeMap::new();
    for statement in statements {
        match statement {
            Syntax::Label(label) => {
                labels.insert(label.clone(), byte_location);
            }
            Syntax::Instruction(instruction) => {
                byte_location += instruction.to_machine_code().len();
            }
        }
    }
    todo!()
}

fn interpret_tokens(src: &[Token]) -> Option<Vec<Syntax>> {
    match src {
        [Token::Keyword(Keyword::Yield), Token::SemiColon, rest @ ..] => Some(add_vecs(
            vec![Syntax::Instruction(Instruction::Yield)],
            interpret_tokens(rest)?,
        )),
        [Token::Label(label), rest @ ..] => Some(add_vecs(
            vec![Syntax::Label(label.clone())],
            interpret_tokens(rest)?,
        )),
        [Token::Keyword(Keyword::Mov), Token::Literal(lit), Token::Address(addr), Token::SemiColon, rest @ ..] => {
            Some(add_vecs(
                vec![Syntax::Instruction(Instruction::Mov(
                    Item::Literal(*lit),
                    addr.clone(),
                ))],
                interpret_tokens(rest)?,
            ))
        }
        [Token::Keyword(Keyword::Mov), Token::Address(src), Token::Address(dst), Token::SemiColon, rest @ ..] => {
            Some(add_vecs(
                vec![Syntax::Instruction(Instruction::Mov(
                    Item::Address(src.clone()),
                    dst.clone(),
                ))],
                interpret_tokens(rest)?,
            ))
        }
        [math_op @ Token::Keyword(
            Keyword::Add
            | Keyword::Sub
            | Keyword::Mul
            | Keyword::And
            | Keyword::Or
            | Keyword::Xor
            | Keyword::Shl
            | Keyword::Shr,
        )] => todo!(),
        _ => None,
    }
}

fn add_vecs<T>(mut a: Vec<T>, b: impl IntoIterator<Item = T>) -> Vec<T> {
    a.extend(b);
    a
}
