use std::collections::BTreeMap;

use crate::utils::print_and_ret;

use super::{
    instruction::{Instruction, Item},
    Keyword, Token,
};

#[derive(Debug)]
pub enum Syntax {
    Label(String),
    Instruction(Instruction),
}

pub(super) fn interpret(src: &[Token]) -> Option<Vec<u16>> {
    // get the syntax
    let statements = interpret_tokens(src)?;
    println!("{statements:?}");
    // first pass to get location of all the labels
    let mut byte_location: usize = 0x8000;
    let mut labels = BTreeMap::new();
    for statement in &statements {
        match statement {
            Syntax::Label(label) => {
                labels.insert(label.clone(), byte_location);
            }
            Syntax::Instruction(instruction) => {
                byte_location += instruction.to_machine_code().len();
            }
        }
    }
    Some(
        statements
            .into_iter()
            .filter_map(|syn| match syn {
                Syntax::Label(_) => None,
                Syntax::Instruction(instr) => Some(instr),
            })
            .flat_map(|instr| instr.with_labels(&labels).to_machine_code())
            .collect(),
    )
}

fn interpret_tokens(src: &[Token]) -> Option<Vec<Syntax>> {
    match src {
        [] => Some(Vec::new()),
        [Token::Keyword(Keyword::Yield), Token::SemiColon, rest @ ..] => Some(add_vecs(
            print_and_ret(vec![Syntax::Instruction(Instruction::Yield)]),
            (interpret_tokens(rest)?),
        )),
        [Token::Label(label), rest @ ..] => Some(add_vecs(
            print_and_ret(vec![Syntax::Label(label.clone())]),
            (interpret_tokens(rest)?),
        )),
        [Token::Keyword(Keyword::Mov), Token::Literal(lit), Token::Address(addr), Token::SemiColon, rest @ ..] => {
            Some(add_vecs(
                print_and_ret(vec![Syntax::Instruction(Instruction::Mov(
                    Item::Literal(lit.clone()),
                    addr.clone(),
                ))]),
                (interpret_tokens(rest)?),
            ))
        }
        [Token::Keyword(Keyword::Mov), Token::Address(src), Token::Address(dst), Token::SemiColon, rest @ ..] => {
            Some(add_vecs(
                print_and_ret(vec![Syntax::Instruction(Instruction::Mov(
                    Item::Address(src.clone()),
                    dst.clone(),
                ))]),
                (interpret_tokens(rest)?),
            ))
        }
        [Token::Keyword(Keyword::Jmp), Token::Literal(lit), Token::SemiColon, rest @ ..] => {
            Some(add_vecs(
                print_and_ret(vec![Syntax::Instruction(Instruction::Jmp(Item::Literal(
                    lit.clone(),
                )))]),
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
