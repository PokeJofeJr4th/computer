use std::collections::BTreeMap;

use crate::{
    asm::instruction::CmpOp,
    utils::{add_vecs, print_and_ret},
};

use super::{
    instruction::{Instruction, Item, MathOp},
    Keyword, Token,
};

#[derive(Debug)]
pub enum Syntax {
    Label(String),
    Instruction(Instruction),
    Literal(u16),
    Reserve(u16),
}

pub(super) fn interpret(src: &[Token]) -> Result<Vec<u16>, Vec<Token>> {
    // get the syntax
    let statements = interpret_tokens(src)?;
    println!("{statements:?}");
    // first pass to get location of all the labels
    let mut byte_location: u16 = 0x8000;
    let mut labels = BTreeMap::new();
    for statement in &statements {
        match statement {
            Syntax::Label(label) => {
                labels.insert(label.clone(), byte_location);
            }
            Syntax::Instruction(instruction) => {
                byte_location += instruction.to_machine_code().len() as u16;
            }
            Syntax::Literal(_) => byte_location += 1,
            Syntax::Reserve(len) => byte_location += len,
        }
    }
    Ok(statements
        .into_iter()
        .flat_map(|syn| match syn {
            Syntax::Label(_) => Vec::new(),
            Syntax::Literal(lit) => vec![lit],
            Syntax::Reserve(lit) => vec![0; lit.into()],
            Syntax::Instruction(instr) => instr.with_labels(&labels).to_machine_code(),
        })
        .collect())
}

#[allow(clippy::too_many_lines)]
fn interpret_tokens(src: &[Token]) -> Result<Vec<Syntax>, Vec<Token>> {
    match src {
        [] => Ok(Vec::new()),
        [Token::Keyword(Keyword::Yield), Token::SemiColon, rest @ ..] => Ok(add_vecs(
            print_and_ret(vec![Syntax::Instruction(Instruction::Yield)]),
            interpret_tokens(rest)?,
        )),
        [Token::Label(label), rest @ ..] => Ok(add_vecs(
            print_and_ret(vec![Syntax::Label(label.clone())]),
            interpret_tokens(rest)?,
        )),
        [Token::Keyword(Keyword::Mov), src @ (Token::Literal(_) | Token::Address(_)), Token::Address(addr), Token::SemiColon, rest @ ..] => {
            Ok(add_vecs(
                print_and_ret(vec![Syntax::Instruction(Instruction::Mov(
                    Item::try_from(src.clone()).unwrap(),
                    addr.clone(),
                ))]),
                interpret_tokens(rest)?,
            ))
        }
        [Token::Keyword(Keyword::Swp), Token::Address(src), Token::Address(dst), Token::SemiColon, rest @ ..] => {
            Ok(add_vecs(
                print_and_ret(vec![Syntax::Instruction(Instruction::Swp(
                    src.clone(),
                    dst.clone(),
                ))]),
                interpret_tokens(rest)?,
            ))
        }
        [Token::Keyword(Keyword::Jmp), jmp @ (Token::Address(_) | Token::Literal(_)), Token::SemiColon, rest @ ..] => {
            Ok(add_vecs(
                print_and_ret(vec![Syntax::Instruction(Instruction::Jmp(
                    Item::try_from(jmp.clone()).unwrap(),
                ))]),
                interpret_tokens(rest)?,
            ))
        }
        [Token::Keyword(cmp @ (Keyword::Jez | Keyword::Jnz)), Token::Address(cnd), jump @ (Token::Address(_) | Token::Literal(_)), Token::SemiColon, rest @ ..] => {
            Ok(add_vecs(
                vec![Syntax::Instruction(Instruction::Jcmpz(
                    *cmp == Keyword::Jez,
                    cnd.clone(),
                    Item::try_from(jump.clone()).unwrap(),
                ))],
                interpret_tokens(rest)?,
            ))
        }
        [Token::Keyword(Keyword::Deref), Token::Address(src), Token::Address(dst), Token::SemiColon, rest @ ..] => {
            Ok(add_vecs(
                vec![Syntax::Instruction(Instruction::Deref(
                    src.clone(),
                    dst.clone(),
                ))],
                interpret_tokens(rest)?,
            ))
        }
        [Token::Keyword(Keyword::Movptr), src @ (Token::Address(_) | Token::Literal(_)), Token::Address(dst), Token::SemiColon, rest @ ..] => {
            Ok(add_vecs(
                vec![Syntax::Instruction(Instruction::Movptr(
                    Item::try_from(src.clone()).unwrap(),
                    dst.clone(),
                ))],
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
            Ok(add_vecs(
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
            Ok(add_vecs(
                vec![Syntax::Instruction(Instruction::JmpCmp(
                    cmp_op,
                    src.clone(),
                    Item::try_from(src_a.clone()).unwrap(),
                    Item::try_from(jmp.clone()).unwrap(),
                ))],
                interpret_tokens(rest)?,
            ))
        }
        [Token::Keyword(
            cmp_op @ (Keyword::Ceq
            | Keyword::Cne
            | Keyword::Clt
            | Keyword::Cle
            | Keyword::Cgt
            | Keyword::Cge),
        ), Token::Address(src), src_a @ (Token::Address(_) | Token::Literal(_)), Token::Address(dst), Token::SemiColon, rest @ ..] =>
        {
            let cmp_op = CmpOp::try_from(*cmp_op).unwrap();
            Ok(add_vecs(
                vec![Syntax::Instruction(Instruction::Cmp(
                    cmp_op,
                    src.clone(),
                    Item::try_from(src_a.clone()).unwrap(),
                    dst.clone(),
                ))],
                interpret_tokens(rest)?,
            ))
        }
        [Token::Keyword(Keyword::Reserve), Token::Literal(lit), Token::SemiColon, rest @ ..] => {
            Ok(add_vecs(
                vec![Syntax::Reserve(lit.to_number())],
                interpret_tokens(rest)?,
            ))
        }
        [Token::Literal(lit), Token::SemiColon, rest @ ..] => Ok(add_vecs(
            vec![Syntax::Literal(lit.to_number())],
            interpret_tokens(rest)?,
        )),
        rest => Err(rest.to_vec()),
    }
}
