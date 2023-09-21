#![warn(clippy::pedantic, clippy::nursery)]

use std::fmt::Debug;

/// # Memory Layout
/// ## Instruction Pointer
/// 0x0000
/// ## General-Purpose Registers
/// 0x0010 - 0x001F
/// ## Screen Registers
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
    pub const INSTRUCTION_PTR: u16 = 0x0010;

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
        let nibbles = (
            instruction >> 12,
            instruction >> 8 & 0xf,
            instruction >> 4 & 0xf,
            instruction & 0xf,
        );
        if nibbles.0 == 0 {
            // MOV/JMP
            if nibbles.1 <= 8 {
                self.mov_or_jmp(nibbles.1, nibbles.2, nibbles.3);
            }
        }
    }

    fn mov_or_jmp(&mut self, mode: u16, first_arg: u16, second_arg: u16) {
        if mode == 0 {
            // MOV &SRC, &DST
            self.set_mem(second_arg, self.get_mem(first_arg));
        } else if mode == 1 {
            // MOV #LIT, &DST
            self.set_mem(second_arg, first_arg);
        } else if mode == 2 {
            // SWP &SRC, &DST
            let inter = self.get_mem(first_arg);
            self.set_mem(first_arg, self.get_mem(second_arg));
            self.set_mem(second_arg, inter);
        } else if mode == 3 {
            // JMP &SRC
            let source = self.get_mem(first_arg);
            self.set_mem(Self::INSTRUCTION_PTR, source);
        } else if mode == 4 {
            // JMP #LIT
            self.set_mem(Self::INSTRUCTION_PTR, first_arg);
        } else if mode == 5 {
            // JEZ &CMP &SRC
            let comparison = self.get_mem(first_arg);
            if comparison == 0 {
                let source = self.get_mem(second_arg);
                self.set_mem(Self::INSTRUCTION_PTR, source);
            }
        } else if mode == 6 {
            // JEZ &CMP #LIT
            let comparison = self.get_mem(first_arg);
            if comparison == 0 {
                self.set_mem(Self::INSTRUCTION_PTR, second_arg);
            }
        } else if mode == 7 {
            // JNZ &CMP &SRC
            let comparison = self.get_mem(first_arg);
            if comparison != 0 {
                let source = self.get_mem(second_arg);
                self.set_mem(Self::INSTRUCTION_PTR, source);
            }
        } else if mode == 8 {
            // JNZ &CMP #LIT
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

    pub fn add_mem(&mut self, idx: u16, value: u16) {
        self.memory[idx as usize] = self.memory[idx as usize].wrapping_add(value);
    }

    pub fn advance_instruction(&mut self, value: u16) {
        self.add_mem(Self::INSTRUCTION_PTR, value);
    }
}
