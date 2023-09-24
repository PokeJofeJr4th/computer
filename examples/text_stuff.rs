use computer::Computer;

const PROGRAM_LOCATION: u16 = 0x8000;
const LOWER_LOCATION: u16 = 0xA000;
const UPPER_LOCATION: u16 = 0xC000;

const LOWER_A: u16 = 97;
const LOWER_Z: u16 = 122;
const UPPER_A: u16 = 65;
const UPPER_Z: u16 = 90;
const ZERO: u16 = 48;
const NINE: u16 = 57;

fn prog() -> Computer {
    let mut comp = Computer::new();
    comp.insert_data(
        PROGRAM_LOCATION,
        &[
            // loop through input
            // get_input: PROGRAM_LOCATION
            //   YIELD
            0x0A00,
            //   (exit loop on zero terminator)
            //   JEZ r0, &print_lower
            0x0D60,
            PROGRAM_LOCATION + 27,
            //   JLT r0, UPPER_A, &get_input
            0x6C30,
            UPPER_A,
            PROGRAM_LOCATION,
            //   JGT r0, UPPER_Z, &check_lower
            0x8C30,
            UPPER_Z,
            PROGRAM_LOCATION + 15,
            //   (it's uppercase)
            //   MOV r0, UPPER_LOCATION
            0x0D00,
            UPPER_LOCATION,
            //   ADD 1, *UPPER_LOCATION
            0x1C11,
            PROGRAM_LOCATION + 10,
            //   JMP &get_input
            0x0E40,
            PROGRAM_LOCATION,
            // check_lower: PROGRAM_LOCATION + 15
            //   JLT r0, LOWER_A, &get_input
            0x6C30,
            LOWER_A,
            PROGRAM_LOCATION,
            //   JGT r0, LOWER_Z, &get_input
            0x8C30,
            LOWER_Z,
            PROGRAM_LOCATION,
            //   (it's uppercase)
            //   MOV r0, LOWER_LOCATION
            0x0D00,
            LOWER_LOCATION,
            //   ADD 1, *LOWER_LOCATION
            0x1C11,
            PROGRAM_LOCATION + 22,
            //   JMP &get_input
            0x0E40,
            PROGRAM_LOCATION,
            // print_lower: PROGRAM_LOCATION + 27
            //   YIELD
            0x0A00,
            //   JEZ LOWER_LOCATION, &print_upper
            0x0F60,
            LOWER_LOCATION,
            PROGRAM_LOCATION + 39,
            //   MOV LOWER_LOCATION, r0
            0x0E00,
            LOWER_LOCATION,
            //   ADD 1, *LOWER_LOCATION
            0x1C11,
            PROGRAM_LOCATION + 29,
            0x1C11,
            PROGRAM_LOCATION + 32,
            //   JMP print_lower
            0x0E40,
            PROGRAM_LOCATION + 27,
            // print_upper: PROGRAM_LOCATION + 39
            //   YIELD
            0x0A00,
            //   JEZ UPPER_LOCATION, #finish
            0x0F60,
            UPPER_LOCATION,
            PROGRAM_LOCATION + 51,
            //   MOV &UPPER_LOCATION, r0
            0x0E00,
            UPPER_LOCATION,
            //   ADD 1, *UPPER_LOCATION
            0x1C11,
            PROGRAM_LOCATION + 41,
            0x1C11,
            PROGRAM_LOCATION + 44,
            //   JMP print_upper
            0x0E40,
            PROGRAM_LOCATION + 39,
            // finish: PROGRAM_LOCATION + 51
            //   MOV #0, r0
            0x0100,
            //   YIELD
            0x0A00,
        ],
    );
    comp.set_mem(Computer::INSTRUCTION_PTR, PROGRAM_LOCATION);
    comp
}

fn comp_println(comp: &mut Computer) {
    let mut str_buf = String::new();
    loop {
        comp.until_yield();
        println!("{str_buf}");
        println!("{comp:?}");
        let char = char::from_u32(comp.get_mem(0x0).into()).unwrap_or_default();
        if char == 0 as char || char == '\n' {
            break;
        }
        str_buf.push(char);
    }
    println!("{str_buf}");
}

fn main() {
    let mut comp = prog();

    let input = String::from("woLoLo");

    // input all the data
    comp.until_yield();
    for k in input.chars() {
        comp.set_mem(0x0000, k as u16);
        comp.until_yield();
        // println!("{comp:?}");
    }

    // add null terminator
    println!("Adding Null Terminator");
    comp.set_mem(0x0000, 0x0000);
    comp.until_yield();
    println!("{comp:?}");

    // print the result
    comp_println(&mut comp);
}