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
    pub const ADD_SRC_DEST: u16 = 0x0010;
    pub const ADD_LIT_DEST: u16 = 0x0011;
    pub const ADD_SRCA_SRC_DEST: u16 = 0x0012;
    pub const ADD_LIT_SRC_DEST: u16 = 0x0013;
    pub const INC_PTR: u16 = 0x0018;
    pub const SUB_SRC_DEST: u16 = 0x0020;
    pub const SUB_LIT_DEST: u16 = 0x0021;
    pub const SUB_SRCA_SRC_DEST: u16 = 0x0022;
    pub const SUB_LIT_SRC_DEST: u16 = 0x0023;
    pub const SUB_SRC_LIT_DEST: u16 = 0x0024;
    pub const MUL_SRC_DEST: u16 = 0x0030;
    pub const MUL_LIT_DEST: u16 = 0x0031;
    pub const MUL_SRCA_SRC_DEST: u16 = 0x0032;
    pub const MUL_LIT_SRC_DEST: u16 = 0x0033;

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
            self.advance_instruction(3);
        } else if instruction == Self::MOV_LIT_DEST {
            // MOV $LIT &DEST
            let literal = self.get_mem(instruction_ptr + 1);
            let destination = self.get_mem(instruction_ptr + 2);
            self.set_mem(destination, literal);
            // consume 3 words
            self.advance_instruction(3);
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
            self.add_mem(destination, source_value);
            self.advance_instruction(3);
        } else if instruction == Self::ADD_LIT_DEST {
            // ADD $LIT &DEST
            let literal = self.get_mem(instruction_ptr + 1);
            let destination = self.get_mem(instruction_ptr + 2);
            self.add_mem(destination, literal);
        } else if instruction == Self::ADD_SRCA_SRC_DEST {
            // &DEST = &SRC - &SRCA
            let source_a = self.get_mem(instruction_ptr + 1);
            let source_a_value = self.get_mem(source_a);
            let source = self.get_mem(instruction_ptr + 2);
            let source_value = self.get_mem(source);
            let destination = self.get_mem(instruction_ptr + 3);
            self.set_mem(destination, source_a_value.wrapping_add(source_value));
            self.advance_instruction(4);
        } else if instruction == Self::ADD_LIT_SRC_DEST {
            // &DEST = &SRC + $LIT
            let literal = self.get_mem(instruction_ptr + 1);
            let source = self.get_mem(instruction_ptr + 2);
            let source_value = self.get_mem(source);
            let destination = self.get_mem(instruction_ptr + 3);
            self.set_mem(destination, source_value.wrapping_add(literal));
            self.advance_instruction(4);
        } else if instruction == Self::INC_PTR {
            // &PTR ++
            let pointer = self.get_mem(instruction_ptr + 1);
            self.add_mem(pointer, 1);
            self.advance_instruction(2);
        }
        // TODO: SUB, MUL
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

    pub fn add_mem(&mut self, idx: u16, value: u16) {
        self.memory[idx as usize] = self.memory[idx as usize].wrapping_add(value);
    }

    pub fn advance_instruction(&mut self, value: u16) {
        self.add_mem(Self::INSTRUCTION_PTR, value);
    }
}
