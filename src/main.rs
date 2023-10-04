#![warn(clippy::pedantic, clippy::nursery)]

use std::fs;

use clap::{Parser, Subcommand};
use computer::{compile_asm, Computer, ComputerDebug, ComputerIO, CPU};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    subcommand: SubCommand,
}

#[derive(Clone, Subcommand, Debug)]
enum SubCommand {
    /// run a bytecode program
    Run {
        /// file to load bytecode from
        filename: String,
        /// print the computer's memory at each stage of execution
        #[clap(short, long)]
        debug: bool,
    },
    /// compile an assembly program to bytecode
    CompileAsm {
        /// file to load assembly from
        source: String,
        /// file to output bytecode
        destination: String,
    },
}

const PROGRAM_LOCATION: u16 = 0x8000;

fn main() {
    let args = Args::parse();
    println!("{args:?}");
    match args.subcommand {
        SubCommand::Run { debug, filename } => {
            let read_file: Vec<u16> = fs::read(filename)
                .unwrap()
                .chunks(2)
                .map(|chunk| {
                    ((chunk.get(0).copied().unwrap_or_default() as u16) << 8)
                        | chunk.get(1).copied().unwrap_or_default() as u16
                })
                .collect();
            let mut comp = ComputerIO::new(CPU::new());
            comp.insert_data(PROGRAM_LOCATION, &read_file);
            comp.set_mem(CPU::INSTRUCTION_PTR, PROGRAM_LOCATION);
            if debug {
                comp.debug_until_yield();
            } else {
                comp.until_yield();
            }
        }
        SubCommand::CompileAsm {
            source,
            destination,
        } => {
            let read_file = fs::read_to_string(source).unwrap();
            let asm = compile_asm(&read_file).unwrap();
            fs::write(
                destination,
                asm.into_iter()
                    .flat_map(|b| vec![(b >> 8) as u8, b as u8])
                    .collect::<Vec<u8>>(),
            )
            .unwrap();
        }
    }
}
