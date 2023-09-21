#![warn(clippy::pedantic, clippy::nursery)]

use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr, BitXor, Shl, Shr},
};

/// # Memory Layout
/// ## Instruction Pointer
/// 0x0010
/// ## Yield
/// 0x0011
/// set this register to `0x0001` to yield execution when using `Computer::until_yield()`
/// ## General-Purpose Registers
/// 0x0000 - 0x000F
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
    pub const YIELD_REGISTER: u16 = 0x0011;

    #[must_use]
    pub const fn new() -> Self {
        Self {
            memory: [0; 0x10000],
        }
    }

    pub fn until_yield(&mut self) {
        while self.get_mem(Self::YIELD_REGISTER) == 0 {
            self.tick();
        }
        self.set_mem(Self::YIELD_REGISTER, 0);
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
                // normal thing with 1-2 arguments
                self.advance_instruction(1);
                self.mov_or_jmp(nibbles.1, nibbles.2, nibbles.3);
            } else if nibbles.1 == 0xD {
                // third nibble is mode, fourth nibble is first arg
                let second_arg = self.get_mem(instruction_ptr + 1);
                self.advance_instruction(2);
                self.mov_or_jmp(nibbles.2, nibbles.3, second_arg);
            } else if nibbles.1 == 0xE {
                // third nibble is mode, fourth nibble is second arg
                let first_arg = self.get_mem(instruction_ptr + 1);
                self.advance_instruction(2);
                self.mov_or_jmp(nibbles.2, first_arg, nibbles.3);
            } else if nibbles.1 == 0xF {
                // third nibble is mode, fourth nibble is unused
                let first_arg = self.get_mem(instruction_ptr + 1);
                let second_arg = self.get_mem(instruction_ptr + 2);
                self.advance_instruction(3);
                self.mov_or_jmp(nibbles.2, first_arg, second_arg);
            }
        } else if nibbles.0 == 1 {
            // ADD
            self.math_op_outer(u16::wrapping_add, instruction_ptr, nibbles);
        } else if nibbles.0 == 2 {
            // SUB
            self.math_op_outer(u16::wrapping_sub, instruction_ptr, nibbles);
        } else if nibbles.0 == 3 {
            // MUL
            self.math_op_outer(u16::wrapping_mul, instruction_ptr, nibbles);
        } else if nibbles.0 == 0xA {
            // NOT
            if nibbles.1 == 0 {
                // NOT &SRC
                self.not_instruction(nibbles.2, nibbles.2);
                self.advance_instruction(1);
            } else if nibbles.1 == 1 {
                // NOT &SRC, &DEST
                self.not_instruction(nibbles.2, nibbles.3);
                self.advance_instruction(1);
            } else if nibbles.1 == 2 {
                // NOT &SRC | SRC
                let source = self.get_mem(instruction_ptr + 1);
                self.not_instruction(source, source);
                self.advance_instruction(2);
            } else if nibbles.1 == 3 {
                // NOT &SRC, &DST | DST
                let destination = self.get_mem(instruction_ptr + 1);
                self.not_instruction(nibbles.2, destination);
                self.advance_instruction(2);
            } else if nibbles.1 == 4 {
                // NOT &SRC, &DST | SRC
                let source = self.get_mem(instruction_ptr + 1);
                self.not_instruction(source, nibbles.2);
                self.advance_instruction(2);
            } else if nibbles.1 == 5 {
                // NOT &SRC, &DST | SRC | DST
                let source = self.get_mem(instruction_ptr + 1);
                let destination = self.get_mem(instruction_ptr + 2);
                self.not_instruction(source, destination);
                self.advance_instruction(3);
            }
        } else if nibbles.0 == 0xB {
            // AND
            self.math_op_outer(BitAnd::bitand, instruction_ptr, nibbles);
        } else if nibbles.0 == 0xC {
            // OR
            self.math_op_outer(BitOr::bitor, instruction_ptr, nibbles);
        } else if nibbles.0 == 0xD {
            // XOR
            self.math_op_outer(BitXor::bitxor, instruction_ptr, nibbles);
        } else if nibbles.0 == 0xE {
            // SHL
            self.math_op_outer(Shl::shl, instruction_ptr, nibbles);
        } else if nibbles.0 == 0xF {
            // SHR
            self.math_op_outer(Shr::shr, instruction_ptr, nibbles);
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
            let comparison = self.get_mem(first_arg);
            if comparison != 0 {
                self.set_mem(Self::INSTRUCTION_PTR, second_arg);
            }
        }
    }

    fn math_op_outer<F: Fn(u16, u16) -> u16>(
        &mut self,
        operation: F,
        instruction_ptr: u16,
        nibbles: (u16, u16, u16, u16),
    ) {
        if nibbles.1 <= 4 {
            let third_arg = if nibbles.1 >= 2 {
                self.advance_instruction(2);
                self.get_mem(instruction_ptr + 1)
            } else {
                self.advance_instruction(1);
                0
            };
            self.math_op(operation, nibbles.1, nibbles.2, nibbles.3, third_arg);
        } else if nibbles.1 == 0xC {
            let second_arg = self.get_mem(instruction_ptr + 1);
            self.advance_instruction(1);
            let third_arg = if nibbles.1 >= 2 {
                self.advance_instruction(1);
                self.get_mem(instruction_ptr + 2)
            } else {
                0
            };
            self.math_op(operation, nibbles.2, nibbles.3, second_arg, third_arg);
        } else if nibbles.1 == 0xD {
            let first_arg = self.get_mem(instruction_ptr + 1);
            self.advance_instruction(1);
            let third_arg = if nibbles.1 >= 2 {
                self.advance_instruction(1);
                self.get_mem(instruction_ptr + 2)
            } else {
                0
            };
            self.math_op(operation, nibbles.2, first_arg, nibbles.3, third_arg);
        } else if nibbles.2 == 0xE {
            let first_arg = self.get_mem(instruction_ptr + 1);
            let second_arg = self.get_mem(instruction_ptr + 2);
            self.advance_instruction(2);
            self.math_op(operation, nibbles.2, first_arg, second_arg, nibbles.3);
        }
    }

    fn math_op<F: Fn(u16, u16) -> u16>(
        &mut self,
        operation: F,
        mode: u16,
        first_arg: u16,
        second_arg: u16,
        third_arg: u16,
    ) {
        if mode == 0 {
            let source = self.get_mem(first_arg);
            self.map_mem(second_arg, source, operation);
        } else if mode == 1 {
            self.map_mem(second_arg, first_arg, operation);
        } else if mode == 2 {
            let source_a = self.get_mem(first_arg);
            let source = self.get_mem(second_arg);
            self.set_mem(third_arg, operation(source_a, source));
        }
    }

    fn not_instruction(&mut self, source: u16, destination: u16) {
        let source = self.get_mem(source);
        self.set_mem(destination, !source);
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

    pub fn sub_mem(&mut self, idx: u16, value: u16) {
        self.memory[idx as usize] = self.memory[idx as usize].wrapping_sub(value);
    }

    pub fn map_mem<F: Fn(u16, u16) -> u16>(&mut self, idx: u16, value: u16, func: F) {
        self.memory[idx as usize] = func(self.memory[idx as usize], value);
    }

    pub fn advance_instruction(&mut self, value: u16) {
        self.add_mem(Self::INSTRUCTION_PTR, value);
    }
}
