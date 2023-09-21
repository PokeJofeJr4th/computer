pub enum Instruction {
    MathBinary(MathOp, Item, Address),
    MathTernary(MathOp, Item, Item, Address),
    Cmp(CmpOp, Address, Item, Item),
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
    Address(u16),
    Label(String),
    Literal(u16),
}

pub enum Address {
    Given(u16),
    Label(String),
}
