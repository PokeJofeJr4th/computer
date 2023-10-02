use std::collections::BTreeMap;

use crate::CPU;

use super::{Keyword, Token};

#[derive(Debug)]
pub enum Instruction {
    Yield,
    Mov(Item, Value),
    Jmp(Item),
    Jez(Value, Item),
    Jnz(Value, Item),
    MathBinary(MathOp, Item, Value),
    MathTernary(MathOp, Item, Item, Value),
    JmpCmp(CmpOp, Value, Item, Item),
    Cmp(CmpOp, Value, Item, Value),
    NotUnary(Value),
    NotBinary(Value, Value),
}

#[derive(Debug, Clone, Copy)]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

impl TryFrom<Keyword> for MathOp {
    type Error = ();
    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            Keyword::Add => Ok(Self::Add),
            Keyword::Sub => Ok(Self::Sub),
            Keyword::Mul => Ok(Self::Mul),
            Keyword::And => Ok(Self::And),
            Keyword::Or => Ok(Self::Or),
            Keyword::Xor => Ok(Self::Xor),
            Keyword::Shl => Ok(Self::Shl),
            Keyword::Shr => Ok(Self::Shr),
            _ => Err(()),
        }
    }
}

impl MathOp {
    pub const fn first_nibble(self) -> u16 {
        match self {
            Self::Add => 0x1000,
            Self::Sub => 0x2000,
            Self::Mul => 0x3000,
            Self::And => 0xB000,
            Self::Or => 0xC000,
            Self::Xor => 0xD000,
            Self::Shl => 0xE000,
            Self::Shr => 0xF000,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl TryFrom<Keyword> for CmpOp {
    type Error = ();
    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            Keyword::Ceq | Keyword::Jeq => Ok(Self::Eq),
            Keyword::Cne | Keyword::Jne => Ok(Self::Ne),
            Keyword::Clt | Keyword::Jlt => Ok(Self::Lt),
            Keyword::Cle | Keyword::Jle => Ok(Self::Le),
            Keyword::Cgt | Keyword::Jgt => Ok(Self::Gt),
            Keyword::Cge | Keyword::Jge => Ok(Self::Ge),
            _ => Err(()),
        }
    }
}

impl CmpOp {
    pub const fn first_nibble(self) -> u16 {
        match self {
            Self::Eq => 0x4000,
            Self::Ne => 0x5000,
            Self::Lt => 0x6000,
            Self::Le => 0x7000,
            Self::Gt => 0x8000,
            Self::Ge => 0x9000,
        }
    }
}

#[derive(Debug)]
pub enum Item {
    Address(Value),
    Literal(Value),
}

impl Item {
    pub fn with_labels(self, labels: &BTreeMap<String, usize>) -> Self {
        match self {
            Self::Address(addr) => Self::Address(addr.with_labels(labels)),
            Self::Literal(addr) => Self::Literal(addr.with_labels(labels)),
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn from_token(tok: Token) -> Option<Self> {
        match tok {
            Token::Address(v) => Some(Self::Address(v)),
            Token::Literal(v) => Some(Self::Literal(v)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Given(u16),
    Label(String),
}

impl Default for Value {
    fn default() -> Self {
        Self::Given(0)
    }
}

impl Value {
    pub const fn to_number(&self) -> u16 {
        match self {
            Self::Given(num) => *num,
            Self::Label(_) => u16::MAX,
        }
    }

    pub fn with_labels(self, labels: &BTreeMap<String, usize>) -> Self {
        match self {
            Self::Given(num) => Self::Given(num),
            Self::Label(label) => Self::Given(labels.get(&label).cloned().unwrap() as u16),
        }
    }
}

impl Instruction {
    pub fn to_machine_code(&self) -> Vec<u16> {
        match self {
            Self::Yield => vec![CPU::YIELD_INSTRUCTION],
            Self::Mov(Item::Literal(lit), dst) => match (lit.to_number(), dst.to_number()) {
                (lit @ 0..=0xF, dst @ 0..=0xF) => {
                    vec![0x0100 | (lit << 4) | dst]
                }
                (lit, dst @ 0..=0xF) => {
                    vec![0x0E10 | dst, lit]
                }
                (lit @ 0..=0xF, dst) => {
                    vec![0x0D10 | lit, dst]
                }
                (lit, dst) => vec![0x0F00, lit, dst],
            },
            Self::Mov(Item::Address(src), dst) => {
                let src = src.to_number();
                let dst = dst.to_number();
                match (src, dst) {
                    (src @ 0..=0xF, dst @ 0..=0xF) => {
                        vec![src << 4 | dst]
                    }
                    (src @ 0..=0xF, dst) => {
                        vec![0x0D00 | src, dst]
                    }
                    (src, dst @ 0..=0xF) => {
                        vec![0x0E00 | dst, src]
                    }
                    (src, dst) => {
                        vec![0x0F00, src, dst]
                    }
                }
            }
            Self::Jmp(Item::Literal(lit)) => {
                let lit = lit.to_number();
                if lit <= 0xF {
                    vec![0x0400 | (lit << 4)]
                } else {
                    vec![0x0E40, lit]
                }
            }
            Self::MathBinary(math_op, Item::Address(src), dst) => {
                let src = src.to_number();
                let dst = dst.to_number();
                match (src, dst) {
                    (src @ 0..=0xF, dst @ 0..=0xF) => {
                        vec![math_op.first_nibble() | src << 4 | dst]
                    }
                    (src @ 0..=0xF, dst) => {
                        vec![math_op.first_nibble() | 0x0C00 | src, dst]
                    }
                    (src, dst @ 0..=0xF) => {
                        vec![math_op.first_nibble() | 0x0D00 | dst, src]
                    }
                    (src, dst) => {
                        vec![math_op.first_nibble() | 0x0E00, src, dst]
                    }
                }
            }
            Self::MathBinary(math_op, Item::Literal(lit), dst) => {
                let lit = lit.to_number();
                let dst = dst.to_number();
                match (lit, dst) {
                    (lit @ 0..=0xF, dst @ 0..=0xF) => {
                        vec![math_op.first_nibble() | 0x0100 | lit << 4 | dst]
                    }
                    (lit @ 0..=0xF, dst) => {
                        vec![math_op.first_nibble() | 0x0C10 | lit, dst]
                    }
                    (lit, dst @ 0..=0xF) => {
                        vec![math_op.first_nibble() | 0x0D10 | dst, lit]
                    }
                    (lit, dst) => {
                        vec![math_op.first_nibble() | 0x0E10, lit, dst]
                    }
                }
            }
            _ => Vec::new(),
        }
    }

    pub fn with_labels(self, labels: &BTreeMap<String, usize>) -> Self {
        match self {
            Self::Yield => Self::Yield,
            Self::Mov(a, b) => Self::Mov(a.with_labels(labels), b.with_labels(labels)),
            Self::Jmp(a) => Self::Jmp(a.with_labels(labels)),
            Self::Jez(_, _) => todo!(),
            Self::Jnz(_, _) => todo!(),
            Self::MathBinary(op, a, b) => {
                Self::MathBinary(op, a.with_labels(labels), b.with_labels(labels))
            }
            Self::MathTernary(_, _, _, _) => todo!(),
            Self::JmpCmp(_, _, _, _) => todo!(),
            Self::Cmp(_, _, _, _) => todo!(),
            Self::NotUnary(_) => todo!(),
            Self::NotBinary(_, _) => todo!(),
        }
    }
}
