use crate::CPU;

pub enum Instruction {
    Yield,
    Mov(Item, Address),
    Jmp(Item),
    Jez(Address, Item),
    Jnz(Address, Item),
    MathBinary(MathOp, Item, Address),
    MathTernary(MathOp, Item, Item, Address),
    JmpCmp(CmpOp, Address, Item, Item),
    Cmp(CmpOp, Address, Item, Address),
    NotUnary(Address),
    NotBinary(Address, Address),
}

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

pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

pub enum Item {
    Address(Address),
    Literal(u16),
}

#[derive(Clone, Debug)]
pub enum Address {
    Given(u16),
    Label(String),
}

impl Default for Address {
    fn default() -> Self {
        Self::Given(0)
    }
}

impl Address {
    pub const fn to_number(&self) -> u16 {
        match self {
            Self::Given(num) => *num,
            Self::Label(_) => u16::MAX,
        }
    }
}

impl Instruction {
    pub fn to_machine_code(&self) -> Vec<u16> {
        match self {
            Self::Yield => vec![CPU::YIELD_INSTRUCTION],
            Self::Mov(Item::Literal(lit), dst) => {
                let dst = dst.to_number();
                match (*lit, dst) {
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
                }
            }
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
            _ => Vec::new(),
        }
    }
}
