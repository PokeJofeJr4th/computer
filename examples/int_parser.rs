use computer::{Computer, ComputerDebug, CPU};

const PROGRAM_LOCATION: u16 = 0x8000;

const INPUT_BUFFER: u16 = 0xA000;

const ZERO: u16 = 48;
const NINE: u16 = 57;

fn program() -> CPU {
    let mut comp = CPU::new();
    comp.insert_data(
        PROGRAM_LOCATION,
        &[
            // get_input: +0
            //   YIELD
            CPU::YIELD_INSTRUCTION,
            //   JEZ r0, #parse
            0x0D60,
            PROGRAM_LOCATION + 0xF,
            //   JLT r0, #ZERO, #get_input
            0x6C30,
            ZERO,
            PROGRAM_LOCATION,
            //   JGT r0, #NINE, #get_input
            0x8C30,
            NINE,
            PROGRAM_LOCATION,
            //   MOV r0, INPUT_BUFFER
            0x0D00,
            // *INPUT_BUFFER: +A
            INPUT_BUFFER,
            //   ADD #1, *INPUT_BUFFER
            0x1C11,
            PROGRAM_LOCATION + 0xA,
            //   JMP #get_input
            0x0F40,
            PROGRAM_LOCATION,
            // parse: +F
            //   MOV #1, r1
            0x0111,
            //   MOV *INPUT_BUFFER, *OUTPUT_BUFFER
            0x0F00,
            PROGRAM_LOCATION + 0xA,
            PROGRAM_LOCATION + 0x16,
            //   SUB #1, *OUTPUT_BUFFER
            0x2C11,
            PROGRAM_LOCATION + 0x16,
            // parse_loop: +15
            //   MOV &OUTPUT_BUFFER, r2
            0x0E02,
            // *OUTPUT_BUFFER: +16
            0xFFFF,
            //   JLT r2, #ZERO, #end
            0x6C32,
            ZERO,
            PROGRAM_LOCATION + 0x26,
            //   JGT r2, #NINE, #end
            0x8C32,
            NINE,
            PROGRAM_LOCATION + 0x26,
            //   SUB #ZERO, r2
            0x2D12,
            ZERO,
            //   MUL r1, r2
            0x3012,
            //   ADD r2, r0
            0x1020,
            //   MUL #10, r1
            0x31A1,
            //   SUB #1, *OUTPUT_BUFFER
            0x2C11,
            PROGRAM_LOCATION + 0x16,
            //   JMP #parse_loop
            0x0F40,
            PROGRAM_LOCATION + 0x15,
            // end: +26
            //   YIELD
            CPU::YIELD_INSTRUCTION,
        ],
    );
    comp.set_mem(CPU::INSTRUCTION_PTR, PROGRAM_LOCATION);
    comp
}

fn main() {
    let mut comp = program();

    println!("Enter a string to parse:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // input all the data
    comp.until_yield();
    println!("{comp:?}");
    for k in input.chars() {
        comp.set_mem(0x0000, k as u16);
        comp.debug_until_yield();
        println!("{k}\n{comp:?}");
    }

    // add null terminator
    println!("Adding Null Terminator");
    comp.set_mem(0x0000, 0x0000);
    comp.debug_until_yield();
    println!("{comp:?}");

    // get the output
    println!("{}", comp.get_mem(0x0000));
}
