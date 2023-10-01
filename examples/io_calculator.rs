use computer::{Computer, ComputerDebug, ComputerIO, CPU};

const PROGRAM_LOCATION: u16 = 0x8000;
const PRINT_LOCATION: u16 = 0x8800;

const INPUT_BUFFER: u16 = 0xA000;
const FIRST_NUMBER_BUFFER: u16 = 0xB000;
const SECOND_NUMBER_BUFFER: u16 = 0xB100;
const OPERATION_BUFFER: u16 = 0xB200;

const ZERO: u16 = 48;
const NINE: u16 = 57;

const PLUS: u16 = 43;
const MINUS: u16 = 45;
const STAR: u16 = 42;

fn program() -> ComputerIO<CPU> {
    let mut comp = ComputerIO::new(CPU::new());
    comp.insert_data(
        PROGRAM_LOCATION,
        &[
            // get_input: +0
            //   MOV #1, &SIGNAL_REGISTER
            0x0D11,
            ComputerIO::<CPU>::SIGNAL_REGISTER,
            //   MOV #FIRST_NUMBER_BUFFER, *STRING_LOCATION
            0x0F10,
            FIRST_NUMBER_BUFFER,
            PRINT_LOCATION + 0x3,
            //   MOV #read_input, *CALLBACK_LOCATION
            0x0F10,
            PROGRAM_LOCATION + 0xA,
            PRINT_LOCATION + 0xA,
            //   JMP #PRINT_LOCATION
            0x0E40,
            PRINT_LOCATION,
            // read_input: +A
            //   MOV #2, &SIGNAL_REGISTER
            0x0D12,
            ComputerIO::<CPU>::SIGNAL_REGISTER,
            //   YIELD
            CPU::YIELD_INSTRUCTION,
            //   JEZ r0, #parse
            0x0D60,
            PROGRAM_LOCATION + 0x13,
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
            // *INPUT_BUFFER: +16
            INPUT_BUFFER,
            //   ADD #1, *INPUT_BUFFER
            0x1C11,
            PROGRAM_LOCATION + 0x16,
            //   JMP #get_input
            0x0F40,
            PROGRAM_LOCATION,
            // parse: +1B
            //   MOV #1, r1
            0x0111,
            //   MOV *INPUT_BUFFER, *OUTPUT_BUFFER
            0x0F00,
            PROGRAM_LOCATION + 0x16,
            PROGRAM_LOCATION + 0x22,
            //   SUB #1, *OUTPUT_BUFFER
            0x2C11,
            PROGRAM_LOCATION + 0x22,
            // parse_loop: +21
            //   MOV &OUTPUT_BUFFER, r2
            0x0E02,
            // *OUTPUT_BUFFER: +22
            0xFFFF,
            //   JLT r2, #ZERO, #second_input
            0x6C32,
            ZERO,
            // *SECOND_INPUT: +25
            PROGRAM_LOCATION + 0x32,
            //   JGT r2, #NINE, #end
            0x8C32,
            NINE,
            PROGRAM_LOCATION + 0x5D,
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
            PROGRAM_LOCATION + 0x22,
            //   JMP #parse_loop
            0x0F40,
            PROGRAM_LOCATION + 0x21,
            // second_input: +32
            //   MOV r0, rF
            0x000F,
            //   MOV #INPUT_BUFFER, *INPUT_BUFFER
            0x0F10,
            INPUT_BUFFER,
            PROGRAM_LOCATION + 0x16,
            //   MOV #math, *SECOND_INPUT
            0x0F10,
            PROGRAM_LOCATION + 0x3B,
            PROGRAM_LOCATION + 0x25,
            //   MOV #SECOND_NUMBER_BUFFER, *FIRST_NUMBER_BUFFER
            0x0F10,
            SECOND_NUMBER_BUFFER,
            PROGRAM_LOCATION + 0x3,
            //   JMP #get_input
            0x0E40,
            PROGRAM_LOCATION,
            // math: +3E
            //   MOV r0, rE
            0x000E,
            // math_loop: +3F
            //   MOV #2, &SIGNAL_REGISTER
            0x0D12,
            ComputerIO::<CPU>::SIGNAL_REGISTER,
            //   YIELD
            CPU::YIELD_INSTRUCTION,
            //   JEZ r0, #end
            0x0D60,
            PROGRAM_LOCATION + 0x5D,
            //   JEQ r0, #PLUS, #add
            0x4C30,
            PLUS,
            PROGRAM_LOCATION + 0x4F,
            //   JEQ r0, #MINUS, #sub
            0x4C30,
            MINUS,
            PROGRAM_LOCATION + 0x52,
            //   JEQ r0, #STAR, #mul
            0x4C30,
            STAR,
            PROGRAM_LOCATION + 0x55,
            //   JMP #math_loop
            0x0E40,
            PROGRAM_LOCATION + 0x3F,
            // add: +4F
            //   ADD rE, rF
            0x10EF,
            //   JMP #after_math
            0x0E40,
            PROGRAM_LOCATION + 0x56,
            // sub: +52
            //   SUB rE, rF
            0x20EF,
            //   JMP #after_math
            0x0E40,
            PROGRAM_LOCATION + 0x56,
            // mul: +55
            //   MUL rE, rF
            0x30EF,
            // after_math: +56
            //   YIELD
            CPU::YIELD_INSTRUCTION,
            //   JNZ r0, #after_math
            0x0D80,
            PROGRAM_LOCATION + 0x56,
            //   MOV rF, r0
            0x00F0,
            //   YIELD
            CPU::YIELD_INSTRUCTION,
            //   JMP #second_input
            0x0E40,
            PROGRAM_LOCATION + 0x35,
            // end: +5D
            //   YIELD
            CPU::YIELD_INSTRUCTION,
            //   MOV #0, r0
            0x0100,
            //   JMP #end
            0x0F40,
            PROGRAM_LOCATION + 0x5D,
        ],
    );

    comp.insert_data(
        PRINT_LOCATION,
        &[
            // MOV #1, &SIGNAL_REGISTER
            0x0D11,
            ComputerIO::<CPU>::SIGNAL_REGISTER,
            // print_loop: +2
            //   MOV &STRING_LOCATION, r0
            0x0E00,
            // *STRING_LOCATION: +3
            0xFFFF,
            //   ADD #1, *STRING_LOCATION
            0x1C11,
            PRINT_LOCATION + 0x3,
            //   YIELD
            CPU::YIELD_INSTRUCTION,
            //   JNZ r0, #print_loop
            0x0D80,
            PRINT_LOCATION + 0x2,
            // JMP #callback_location
            0x0E40,
            // *CALLBACK_LOCATION: +A
            0xFFFF,
        ],
    );

    comp.insert_string(FIRST_NUMBER_BUFFER, "Enter the first number:");
    comp.insert_string(SECOND_NUMBER_BUFFER, "Enter the next number:");
    comp.insert_string(OPERATION_BUFFER, "Enter the operation (+, -, or *):");
    comp.set_mem(CPU::INSTRUCTION_PTR, PROGRAM_LOCATION);
    comp
}

fn main() {
    let mut comp = program();
    // comp.debug_until_yield();
    comp.until_yield();
}
