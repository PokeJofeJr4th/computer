#![warn(clippy::pedantic, clippy::nursery)]

use std::fmt::Debug;

mod asm;

/// # Memory Layout
/// ## Instruction Pointer - 0x0000 - 0x0001
/// ## Screen Registers
/// ## Stack
/// ## Heap
pub struct Computer {
    memory: [u16; 0x10000],
}

impl Debug for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..0x1000 {
            write!(f, "{:0>4X}", x << 4)?;
            for b in 0..0x10 {
                let idx = (x << 4) + b;
                let mem = self.get_mem(idx);
                write!(f, " {mem:0>4X}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Computer {
    pub const INSTRUCTION_PTR: u16 = 0;
    pub const INSTRUCTION_PTR_USIZE: usize = Self::INSTRUCTION_PTR as usize;

    pub const MOV_SRC_DEST: u16 = 0x0000;
    pub const MOV_LIT_DEST: u16 = 0x0001;
    pub const JMP_SRC: u16 = 0x0002;
    pub const JMP_LIT: u16 = 0x0003;
    pub const ADD_SRC_DEST: u16 = 0x0004;
    pub const ADD_LIT_DEST: u16 = 0x0005;
    pub const ADD_SRCA_SRC_DEST: u16 = 0x0006;
    pub const ADD_LIT_SRC_DEST: u16 = 0x0007;

    #[must_use]
    pub const fn new() -> Self {
        Self {
            memory: [0; 0x10000],
        }
    }

    pub fn tick(&mut self) {
        let instruction_ptr = self.get_mem(Self::INSTRUCTION_PTR);
        if instruction_ptr == Self::INSTRUCTION_PTR {
            return;
        }
        let instruction = self.get_mem(instruction_ptr);
        if instruction == Self::MOV_SRC_DEST {
            // MOV &SRC &DEST
            let source = self.get_mem(instruction_ptr + 1);
            let source_value = self.get_mem(source);
            let destination = self.get_mem(instruction_ptr + 2);
            self.set_mem(destination, source_value);
            // consume 3 words
            *self.mut_mem(Self::INSTRUCTION_PTR) += 3;
        } else if instruction == Self::MOV_LIT_DEST {
            // MOV $LIT &DEST
            let literal = self.get_mem(instruction_ptr + 1);
            let destination = self.get_mem(instruction_ptr + 2);
            self.set_mem(destination, literal);
            // consume 3 words
            *self.mut_mem(Self::INSTRUCTION_PTR) += 3;
        } else if instruction == Self::JMP_LIT {
            // JMP $LIT
            let address = self.get_mem(instruction_ptr + 1);
            self.set_mem(Self::INSTRUCTION_PTR, address);
            // don't need to consume any words, since we're just moving the pointer anyway
        } else if instruction == Self::JMP_SRC {
            // JMP &SRC
            let source = self.get_mem(instruction_ptr + 1);
            let source_value = self.get_mem(source);
            self.set_mem(Self::INSTRUCTION_PTR, source_value);
            // don't need to consume any words, since we're just moving the pointer anyway
        } else if instruction == Self::ADD_SRC_DEST {
            // ADD &SRC &DEST
            let source = self.get_mem(instruction_ptr + 1);
            let source_value = self.get_mem(source);
            let destination = self.get_mem(instruction_ptr + 2);
            let destination_value = self.get_mem(destination);
            let sum_value = source_value + destination_value;
            self.set_mem(destination, sum_value);
            // consume 3 words
            *self.mut_mem(Self::INSTRUCTION_PTR) += 3;
        } else if instruction == Self::ADD_LIT_DEST {
            // ADD $LIT &DEST
            let literal = self.get_mem(instruction_ptr + 1);
            let destination = self.get_mem(instruction_ptr + 2);
            *self.mut_mem(destination) += literal;
        }
    }

    #[must_use]
    pub const fn get_mem(&self, idx: u16) -> u16 {
        self.memory[idx as usize]
    }

    pub fn set_mem(&mut self, idx: u16, value: u16) {
        self.memory[idx as usize] = value;
    }

    pub fn mut_mem(&mut self, idx: u16) -> &mut u16 {
        &mut self.memory[idx as usize]
    }
}
