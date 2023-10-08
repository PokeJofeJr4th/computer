use std::{collections::BTreeMap, fmt::Display, rc::Rc};

use strum::AsRefStr;

use crate::CPU;

use super::{Keyword, Token};

#[derive(Debug)]
pub enum Instruction {
    Yield,
    Mov(Item, Value),
    Swp(Value, Value),
    Jmp(Item),
    Jcmpz(bool, Value, Item),
    Ptrread(Value, Value),
    Ptrwrite(Item, Value),
    MathBinary(MathOp, Item, Value),
    MathTernary(MathOp, Item, Item, Value),
    JmpCmp(CmpOp, Value, Item, Item),
    Cmp(CmpOp, Value, Item, Value),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yield => write!(f, "YIELD;"),
            Self::Mov(src, dst) => write!(f, "MOV {src} &{dst};"),
            Self::Swp(src, dst) => write!(f, "SWP &{src} &{dst};"),
            Self::Jmp(dst) => write!(f, "JMP {dst};"),
            Self::Jcmpz(is_eq, src, jmp) => {
                write!(f, "J{}Z &{src} {jmp};", if *is_eq { 'E' } else { 'N' })
            }
            Self::Ptrread(src, dst) => write!(f, "PTRREAD &{src} &{dst};"),
            Self::Ptrwrite(src, dst) => write!(f, "PTRWRITE {src} &{dst};"),
            Self::MathBinary(math_op, src, dst) => write!(f, "{} {src} &{dst};", math_op.as_ref()),
            Self::MathTernary(math_op, src, srca, dst) => {
                write!(f, "{} {src} {srca} &{dst}", math_op.as_ref())
            }
            Self::JmpCmp(cmp_op, src, srca, jmp) => {
                write!(f, "J{} &{src} {srca} {jmp};", cmp_op.as_ref())
            }
            Self::Cmp(cmp_op, src, srca, dst) => {
                write!(f, "C{} &{src} {srca} &{dst};", cmp_op.as_ref())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, AsRefStr)]
#[strum(serialize_all = "UPPERCASE")]
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

#[derive(Debug, Clone, Copy, AsRefStr)]
#[strum(serialize_all = "UPPERCASE")]
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

    pub const fn inverse(self) -> Self {
        match self {
            Self::Eq => Self::Ne,
            Self::Ne => Self::Eq,
            Self::Lt => Self::Ge,
            Self::Le => Self::Gt,
            Self::Gt => Self::Le,
            Self::Ge => Self::Lt,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Address(Value),
    Literal(Value),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Address(v) => write!(f, "&{v}"),
            Self::Literal(v) => write!(f, "#{v}"),
        }
    }
}

impl Item {
    pub fn with_labels(self, labels: &BTreeMap<Rc<str>, u16>) -> Self {
        match self {
            Self::Address(addr) => Self::Address(addr.with_labels(labels)),
            Self::Literal(lit) => Self::Literal(lit.with_labels(labels)),
        }
    }

    pub const fn to_number(&self) -> u16 {
        match self {
            Self::Address(addr) => addr.to_number(),
            Self::Literal(lit) => lit.to_number(),
        }
    }
}

impl TryFrom<Token> for Item {
    type Error = ();
    fn try_from(tok: Token) -> Result<Self, ()> {
        match tok {
            Token::Address(v) => Ok(Self::Address(v)),
            Token::Literal(v) => Ok(Self::Literal(v)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Given(u16),
    Label(Rc<str>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Given(v) => write!(f, "{v:0>4X}"),
            Self::Label(v) => write!(f, "{v}"),
        }
    }
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

    pub fn with_labels(self, labels: &BTreeMap<Rc<str>, u16>) -> Self {
        match self {
            Self::Given(num) => Self::Given(num),
            Self::Label(label) => Self::Given(
                labels
                    .get(&label)
                    .copied()
                    .unwrap_or_else(|| panic!("No valid label found for {label}")),
            ),
        }
    }
}

impl Instruction {
    #[allow(clippy::too_many_lines)]
    pub fn to_machine_code(&self) -> Vec<u16> {
        match self {
            Self::Yield => vec![CPU::YIELD_INSTRUCTION],
            Self::Mov(src, dst) => {
                let mode = match src {
                    Item::Address(_) => 0,
                    Item::Literal(_) => 1,
                };
                match (src.to_number(), dst.to_number()) {
                    (lit @ 0..=0xF, dst @ 0..=0xF) => {
                        vec![mode << 8 | (lit << 4) | dst]
                    }
                    (lit, dst @ 0..=0xF) => {
                        vec![0x0E00 | mode << 4 | dst, lit]
                    }
                    (lit @ 0..=0xF, dst) => {
                        vec![0x0D00 | mode << 4 | lit, dst]
                    }
                    (lit, dst) => vec![0x0F00 | mode << 4, lit, dst],
                }
            }
            Self::Swp(src, dst) => match (src.to_number(), dst.to_number()) {
                (src @ 0..=0xF, dst @ 0..=0xF) => {
                    vec![0x0200 | src << 4 | dst]
                }
                (src @ 0..=0xF, dst) => {
                    vec![0x0D20 | src, dst]
                }
                (src, dst @ 0..=0xF) => {
                    vec![0x0E20 | dst, src]
                }
                (src, dst) => {
                    vec![0x0F20, src, dst]
                }
            },
            Self::Jmp(jmp) => {
                let mode = match jmp {
                    Item::Address(_) => 3,
                    Item::Literal(_) => 4,
                };
                let jmp = jmp.to_number();
                if jmp <= 0xF {
                    vec![mode << 8 | (jmp << 4)]
                } else {
                    vec![0x0E00 | mode << 4, jmp]
                }
            }
            Self::Jcmpz(cmp, cnd, jump) => {
                let mode = match (cmp, jump) {
                    (true, Item::Address(_)) => 5,
                    (true, Item::Literal(_)) => 6,
                    (false, Item::Address(_)) => 7,
                    (false, Item::Literal(_)) => 8,
                };
                match (cnd.to_number(), jump.to_number()) {
                    (cnd @ 0..=0xF, jump @ 0..=0xF) => {
                        vec![mode << 8 | cnd << 4 | jump]
                    }
                    (cnd @ 0..=0xF, jump) => {
                        vec![0x0D00 | mode << 4 | cnd, jump]
                    }
                    (cnd, jump @ 0..=0xF) => {
                        vec![0x0E00 | mode << 4 | jump, cnd]
                    }
                    (cnd, jump) => {
                        vec![0x0F00 | mode << 4, cnd, jump]
                    }
                }
            }
            Self::Ptrread(src, dst) => {
                let src = src.to_number();
                let dst = dst.to_number();
                if src == dst {
                    match src {
                        src @ 0..=0xF => {
                            vec![0xA000 | src << 4]
                        }
                        src => {
                            vec![0xAE00, src]
                        }
                    }
                } else {
                    match (src, dst) {
                        (src @ 0..=0xF, dst @ 0..=0xF) => {
                            vec![0xA100 | src << 4 | dst]
                        }
                        (src @ 0..=0xF, dst) => {
                            vec![0xAD10 | src, dst]
                        }
                        (src, dst @ 0..=0xF) => {
                            vec![0xAE10 | dst, src]
                        }
                        (src, dst) => {
                            vec![0xAF10, src, dst]
                        }
                    }
                }
            }
            Self::Ptrwrite(src, dst) => {
                let mode = match src {
                    Item::Address(_) => 2,
                    Item::Literal(_) => 3,
                };
                match (src.to_number(), dst.to_number()) {
                    (src @ 0..=0xF, dst @ 0..=0xF) => {
                        vec![0xA000 | mode << 8 | src << 4 | dst]
                    }
                    (src @ 0..=0xF, dst) => {
                        vec![0xAD00 | mode << 4 | src, dst]
                    }
                    (src, dst @ 0..=0xF) => {
                        vec![0xAE00 | mode << 4 | dst, src]
                    }
                    (src, dst) => {
                        vec![0xAF00 | mode << 4, src, dst]
                    }
                }
            }
            Self::MathBinary(math_op, src, dst) => {
                let mode = match src {
                    Item::Address(_) => 0,
                    Item::Literal(_) => 1,
                };
                let src = src.to_number();
                let dst = dst.to_number();
                match (src, dst) {
                    (src @ 0..=0xF, dst @ 0..=0xF) => {
                        vec![math_op.first_nibble() | mode << 8 | src << 4 | dst]
                    }
                    (src @ 0..=0xF, dst) => {
                        vec![math_op.first_nibble() | 0x0C00 | mode << 4 | src, dst]
                    }
                    (src, dst @ 0..=0xF) => {
                        vec![math_op.first_nibble() | 0x0D00 | mode << 4 | dst, src]
                    }
                    (src, dst) => {
                        vec![math_op.first_nibble() | 0x0E00 | mode << 4, src, dst]
                    }
                }
            }
            Self::MathTernary(math_op, src_a, src, dst) => {
                let mode = match (src_a, src) {
                    (Item::Address(_), Item::Address(_)) => 2,
                    (Item::Literal(_), Item::Address(_)) => 3,
                    (Item::Address(_), Item::Literal(_)) => 4,
                    _ => panic!("Invalid use of ternary math op like `ADD #LIT #LIT &DST`"),
                };
                match (src_a.to_number(), src.to_number(), dst.to_number()) {
                    (src_a @ 0..=0xF, src @ 0..=0xF, dst) => {
                        vec![math_op.first_nibble() | mode << 8 | src_a << 4 | src, dst]
                    }
                    (src_a @ 0..=0xF, src, dst) => {
                        vec![
                            math_op.first_nibble() | 0x0C00 | mode << 4 | src_a,
                            src,
                            dst,
                        ]
                    }
                    (src_a, src @ 0..=0xF, dst) => {
                        vec![
                            math_op.first_nibble() | 0x0D00 | mode << 4 | src,
                            src_a,
                            dst,
                        ]
                    }
                    (src_a, src, dst @ 0..=0xF) => {
                        vec![
                            math_op.first_nibble() | 0x0E00 | mode << 4 | dst,
                            src,
                            src_a,
                        ]
                    }
                    (src_a, src, dst) => {
                        vec![math_op.first_nibble() | 0x0F00 | mode << 4, src_a, src, dst]
                    }
                }
            }
            Self::JmpCmp(cmp_op, src, src_a, jmp) => {
                let mode = match (src_a, jmp) {
                    (Item::Address(_), Item::Address(_)) => 0,
                    (Item::Address(_), Item::Literal(_)) => 1,
                    (Item::Literal(_), Item::Address(_)) => 2,
                    (Item::Literal(_), Item::Literal(_)) => 3,
                };
                match (src.to_number(), src_a.to_number(), jmp.to_number()) {
                    (src @ 0..=0xF, src_a @ 0..=0xF, jmp) => {
                        vec![cmp_op.first_nibble() | (mode << 8) | (src << 4) | src_a, jmp]
                    }
                    (src @ 0..=0xF, src_a, jmp) => {
                        vec![cmp_op.first_nibble() | 0x0C00 | (mode << 4) | src, src_a, jmp]
                    }
                    (src, src_a @ 0..=0xF, jmp) => {
                        vec![cmp_op.first_nibble() | 0x0D00 | (mode << 4) | src_a, src, jmp]
                    }
                    (src, src_a, jmp @ 0..=0xF) => {
                        vec![cmp_op.first_nibble() | 0x0E00 | (mode << 4) | jmp, src, src_a]
                    }
                    (src, src_a, jmp) => {
                        vec![cmp_op.first_nibble() | 0x0F00 | (mode << 4), src, src_a, jmp]
                    }
                }
            }
            Self::Cmp(cmp_op, src, src_a, dst) => {
                let mode = match src_a {
                    Item::Address(_) => 4,
                    Item::Literal(_) => 5,
                };
                match (src.to_number(), src_a.to_number(), dst.to_number()) {
                    (src @ 0..=0xF, src_a @ 0..=0xF, dst) => {
                        vec![cmp_op.first_nibble() | mode << 8 | src << 4 | src_a, dst]
                    }
                    (src @ 0..=0xF, src_a, dst) => {
                        vec![cmp_op.first_nibble() | 0x0C00 | mode << 4 | src, src_a, dst]
                    }
                    (src, src_a @ 0..=0xF, dst) => {
                        vec![cmp_op.first_nibble() | 0x0D00 | mode << 4 | src_a, src, dst]
                    }
                    (src, src_a, dst @ 0..=0xF) => {
                        vec![cmp_op.first_nibble() | 0x0E00 | mode << 4 | dst, src, src_a]
                    }
                    (src, src_a, dst) => {
                        vec![cmp_op.first_nibble() | 0x0F00 | mode << 4, src, src_a, dst]
                    }
                }
            }
        }
    }

    pub fn with_labels(self, labels: &BTreeMap<Rc<str>, u16>) -> Self {
        match self {
            Self::Yield => Self::Yield,
            Self::Mov(a, b) => Self::Mov(a.with_labels(labels), b.with_labels(labels)),
            Self::Swp(a, b) => Self::Swp(a.with_labels(labels), b.with_labels(labels)),
            Self::Jmp(a) => Self::Jmp(a.with_labels(labels)),
            Self::Jcmpz(a, b, c) => Self::Jcmpz(a, b.with_labels(labels), c.with_labels(labels)),
            Self::Ptrread(a, b) => Self::Ptrread(a.with_labels(labels), b.with_labels(labels)),
            Self::Ptrwrite(a, b) => Self::Ptrwrite(a.with_labels(labels), b.with_labels(labels)),
            Self::MathBinary(op, a, b) => {
                Self::MathBinary(op, a.with_labels(labels), b.with_labels(labels))
            }
            Self::MathTernary(op, a, b, c) => Self::MathTernary(
                op,
                a.with_labels(labels),
                b.with_labels(labels),
                c.with_labels(labels),
            ),
            Self::JmpCmp(op, a, b, c) => Self::JmpCmp(
                op,
                a.with_labels(labels),
                b.with_labels(labels),
                c.with_labels(labels),
            ),
            Self::Cmp(op, a, b, c) => Self::Cmp(
                op,
                a.with_labels(labels),
                b.with_labels(labels),
                c.with_labels(labels),
            ),
        }
    }
}
