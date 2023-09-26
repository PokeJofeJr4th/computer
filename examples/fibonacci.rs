use computer::Computer;

const PROGRAM_LOCATION: u16 = 0x8000;

fn prog() -> Computer {
    let mut comp = Computer::new();
    comp.insert_data(
        PROGRAM_LOCATION,
        &[
            // MOV 0, &r0
            0x0100,
            // MOV 1, &r1
            0x0111,
            // _loop:
            // ADD &r0, &r1
            0x1010,
            // SWP &r0, &r1
            0x0201,
            // YIELD
            Computer::YIELD_INSTRUCTION,
            // JMP #_loop
            0x0E40,
            PROGRAM_LOCATION + 2,
        ],
    );
    comp
}

fn main() {
    let mut comp = prog();

    // start running the program
    comp.set_mem(Computer::INSTRUCTION_PTR, PROGRAM_LOCATION);
    println!("{comp:?}");

    for _ in 0..20 {
        comp.until_yield();
        println!("{}", comp.get_mem(0x0000));

        // comp.tick();
        // println!("{} {}", comp.get_mem(0), comp.get_mem(1));
    }

    // println!("{comp:?}");
}
