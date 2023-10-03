use std::collections::BTreeMap;

use crate::{asm::instruction::CmpOp, utils::print_and_ret};

use super::{
    instruction::{Instruction, Item, MathOp},
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
                labels.insert(label.clone(), byte_location as u16);
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

#[allow(clippy::too_many_lines)]
fn interpret_tokens(src: &[Token]) -> Option<Vec<Syntax>> {
    match src {
        [] => Some(Vec::new()),
        [Token::Keyword(Keyword::Yield), Token::SemiColon, rest @ ..] => Some(add_vecs(
            print_and_ret(vec![Syntax::Instruction(Instruction::Yield)]),
            interpret_tokens(rest)?,
        )),
        [Token::Label(label), rest @ ..] => Some(add_vecs(
            print_and_ret(vec![Syntax::Label(label.clone())]),
            interpret_tokens(rest)?,
        )),
        [Token::Keyword(Keyword::Mov), src @ (Token::Literal(_) | Token::Address(_)), Token::Address(addr), Token::SemiColon, rest @ ..] => {
            Some(add_vecs(
                print_and_ret(vec![Syntax::Instruction(Instruction::Mov(
                    Item::try_from(src.clone()).unwrap(),
                    addr.clone(),
                ))]),
                interpret_tokens(rest)?,
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
        [Token::Keyword(
            math_op @ (Keyword::Add
            | Keyword::Sub
            | Keyword::Mul
            | Keyword::And
            | Keyword::Or
            | Keyword::Xor
            | Keyword::Shl
            | Keyword::Shr),
        ), src @ (Token::Address(_) | Token::Literal(_)), Token::Address(dst), Token::SemiColon, rest @ ..] =>
        {
            let math_op = MathOp::try_from(*math_op).unwrap();
            Some(add_vecs(
                vec![Syntax::Instruction(Instruction::MathBinary(
                    math_op,
                    Item::try_from(src.clone()).unwrap(),
                    dst.clone(),
                ))],
                interpret_tokens(rest)?,
            ))
        }
        [Token::Keyword(
            cmp_op @ (Keyword::Jeq
            | Keyword::Jne
            | Keyword::Jlt
            | Keyword::Jle
            | Keyword::Jgt
            | Keyword::Jge),
        ), Token::Address(src), src_a @ (Token::Address(_) | Token::Literal(_)), jmp @ (Token::Address(_) | Token::Literal(_)), Token::SemiColon, rest @ ..] =>
        {
            let cmp_op = CmpOp::try_from(*cmp_op).unwrap();
            Some(add_vecs(
                vec![Syntax::Instruction(Instruction::JmpCmp(
                    cmp_op,
                    src.clone(),
                    Item::try_from(src_a.clone()).unwrap(),
                    Item::try_from(jmp.clone()).unwrap(),
                ))],
                interpret_tokens(rest)?,
            ))
        }
        _ => None,
    }
}

fn add_vecs<T>(mut a: Vec<T>, b: impl IntoIterator<Item = T>) -> Vec<T> {
    a.extend(b);
    a
}
