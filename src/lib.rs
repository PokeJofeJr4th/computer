#![warn(clippy::pedantic, clippy::nursery)]

mod asm;
mod computer;
mod cpu;
mod stdio;
#[cfg(test)]
mod tests;
mod utils;

pub use asm::compile_asm;
pub use computer::{Computer, ComputerDebug};
pub use cpu::CPU;
pub use stdio::ComputerIO;
