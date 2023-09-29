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
    pub const fn new(comp: CPU) -> Self {
        Self(comp)
    }

    pub fn program_input(&mut self, input: impl Iterator<Item = u16>) {
        // input all the data
        self.0.until_yield();
        for k in input {
            self.0.set_mem(0x0000, k);
            self.0.until_yield();
            println!("{k}");
        }

        // add null terminator
        println!("Adding Null Terminator");
        self.0.set_mem(0x0000, 0x0000);
        self.0.until_yield();
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
            match self.get_mem(0x0002) {
                0 => return,
                1 => {
                    let mut value = String::new();
                    loop {
                        if self.0.get_mem(0) == 0 {
                            break;
                        }
                        value.push(u32::from(self.0.get_mem(0)).try_into().unwrap_or('_'));
                    }
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
                }
                _ => {}
            }
        }
    }
}

impl<CPU: ComputerDebug> ComputerIO<CPU> {
    pub fn program_input_debug(&mut self, input: impl Iterator<Item = u16>) {
        // input all the data
        self.0.debug_until_yield();
        for k in input {
            self.0.set_mem(0x0000, k);
            self.0.debug_until_yield();
            println!("{k}\n{self:?}");
        }

        // add null terminator
        println!("Adding Null Terminator");
        self.0.set_mem(0x0000, 0x0000);
        self.0.debug_until_yield();
    }
}
