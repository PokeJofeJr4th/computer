use std::fmt::Debug;

use crate::{Computer, ComputerDebug};

#[derive(Default)]
pub struct ComputerIO<CPU: Computer>(CPU);

impl<CPU: ComputerDebug> Debug for ComputerIO<CPU> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<CPU: Computer> ComputerIO<CPU> {
    pub const SIGNAL_REGISTER: u16 = 0x0012;

    pub const fn new(comp: CPU) -> Self {
        Self(comp)
    }

    pub fn program_input(&mut self, input: impl Iterator<Item = u16>) {
        // input all the data
        for k in input {
            self.0.set_mem(0x0000, k);
            self.0.until_yield();
        }

        // add null terminator
        self.0.set_mem(0x0000, 0x0000);
        // self.0.until_yield();
    }
}

impl<CPU: Computer> Computer for ComputerIO<CPU> {
    fn get_mem(&self, idx: u16) -> u16 {
        self.0.get_mem(idx)
    }

    fn insert_data(&mut self, idx: impl Into<usize>, data: &[u16]) {
        self.0.insert_data(idx, data);
    }

    fn set_mem(&mut self, idx: u16, value: u16) {
        self.0.set_mem(idx, value);
    }

    fn until_yield(&mut self) {
        loop {
            self.0.until_yield();
            match self.0.get_mem(Self::SIGNAL_REGISTER) {
                1 => {
                    let mut value = String::new();
                    loop {
                        if self.0.get_mem(0) == 0 {
                            break;
                        }
                        value.push(u32::from(self.0.get_mem(0)).try_into().unwrap_or('_'));
                        self.0.until_yield();
                    }
                    self.0.set_mem(Self::SIGNAL_REGISTER, 0);
                    println!("{value}");
                }
                2 => {
                    let mut value = String::new();
                    std::io::stdin().read_line(&mut value).unwrap();
                    self.program_input(
                        value
                            .chars()
                            .map(u32::from)
                            .map(u16::try_from)
                            .map(Result::unwrap_or_default)
                            .take_while(|&x| x > 0),
                    );
                    self.0.set_mem(Self::SIGNAL_REGISTER, 0);
                }
                _ => return,
            }
        }
    }
}

impl<CPU: ComputerDebug> ComputerIO<CPU> {
    pub fn program_input_debug(&mut self, input: impl Iterator<Item = u16>) {
        // input all the data
        for k in input {
            self.0.set_mem(0x0000, k);
            self.0.debug_until_yield();
            println!("{k:0>4X}\n{self:?}");
        }

        // add null terminator
        println!("Adding Null Terminator");
        self.0.set_mem(0x0000, 0x0000);
        // self.0.debug_until_yield();
    }
}

impl<CPU: ComputerDebug> ComputerDebug for ComputerIO<CPU> {
    fn debug_until_yield(&mut self) {
        loop {
            self.0.debug_until_yield();
            match self.0.get_mem(Self::SIGNAL_REGISTER) {
                1 => {
                    let mut value = String::new();
                    loop {
                        if self.0.get_mem(0) == 0 {
                            break;
                        }
                        value.push(u32::from(self.0.get_mem(0)).try_into().unwrap_or('_'));
                        self.0.debug_until_yield();
                    }
                    self.0.set_mem(Self::SIGNAL_REGISTER, 0);
                    println!("{value}");
                }
                2 => {
                    let mut value = String::new();
                    std::io::stdin().read_line(&mut value).unwrap();
                    self.program_input_debug(
                        value
                            .chars()
                            .map(u32::from)
                            .flat_map(u16::try_from)
                            .take_while(|&x| x != 0),
                    );
                    self.0.set_mem(Self::SIGNAL_REGISTER, 0);
                }
                _ => return,
            }
        }
    }
}
