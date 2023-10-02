use std::collections::BTreeMap;

use crate::CPU;

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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
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
            Self::MathBinary(_, _, _) => todo!(),
            Self::MathTernary(_, _, _, _) => todo!(),
            Self::JmpCmp(_, _, _, _) => todo!(),
            Self::Cmp(_, _, _, _) => todo!(),
            Self::NotUnary(_) => todo!(),
            Self::NotBinary(_, _) => todo!(),
        }
    }
}
