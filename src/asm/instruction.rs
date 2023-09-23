pub enum Instruction {
    Yield,
    MathBinary(MathOp, Item, Address),
    MathTernary(MathOp, Item, Item, Address),
    JmpCmp(CmpOp, Address, Item, Item),
    Cmp(CmpOp, Address, Item, Address),
    Mov(Item, Address),
    Jmp(Item),
    Jez(Address, Item),
    Jnz(Address, Item),
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

pub enum Address {
    Given(u16),
    Label(String),
}

impl Address {
    pub const fn to_number(&self) -> u16 {
        match self {
            Self::Given(num) => *num,
            Self::Label(_) => 0,
        }
    }
}

impl Instruction {
    pub fn to_machine_code(&self) -> Vec<u16> {
        match self {
            Self::Yield => vec![0x0A00],
            Self::Mov(Item::Literal(lit), dst) => {
                let dst = dst.to_number();
                match (*lit, dst) {
                    (lit @ 0..=0xF, dst @ 0..=0xF) => {
                        vec![(lit << 4) | dst]
                    }
                    (lit, dst) => vec![0x0F00, lit, dst],
                }
            }
            _ => Vec::new(),
        }
    }
}
