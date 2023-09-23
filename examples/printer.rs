use computer::Computer;

const PROGRAM_LOCATION: u16 = 0x8000;

fn prog() -> Computer {
    let mut comp = Computer::new();
    comp.insert_data(
        PROGRAM_LOCATION,
        &[
            // MOV #F, r0
            0x01F0,
            // :_loop
            // MOV r0, rF
            0x000F,
            // YIELD,
            0x0A00,
            // ADD 1, r0
            0x1110,
            // JMP #_loop
            0x0E40,
            PROGRAM_LOCATION + 1,
        ],
    );
    comp
}

fn comp_println(comp: &mut Computer) {
    let mut str_buf = String::new();
    loop {
        comp.until_yield();
        let char = comp.get_mem(0xF).try_into().unwrap_or(b'\n') as char;
        if char == '\n' {
            break;
        }
        str_buf.push(char);
    }
    println!("{str_buf}");
}

fn main() {
    let mut comp = prog();
    comp.set_mem(Computer::INSTRUCTION_PTR, PROGRAM_LOCATION);
    comp_println(&mut comp);
}
