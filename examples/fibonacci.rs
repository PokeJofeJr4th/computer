use computer::Computer;

const PROGRAM_LOCATION: u16 = 0x8000;

fn prog() -> Computer {
    let mut comp = Computer::new();
    // MOV 0, &r0
    comp.set_mem(PROGRAM_LOCATION, 0x0100);
    // MOV 1, &r1
    comp.set_mem(PROGRAM_LOCATION + 1, 0x0111);
    // _loop:
    // ADD &r0, &r1
    comp.set_mem(PROGRAM_LOCATION + 2, 0x1010);
    // SWP &r0, &r1
    comp.set_mem(PROGRAM_LOCATION + 3, 0x0201);
    // MOV 1, &YIELD_REGISTER
    comp.set_mem(PROGRAM_LOCATION + 4, 0x0D11);
    comp.set_mem(PROGRAM_LOCATION + 5, Computer::YIELD_REGISTER);
    // JMP #_loop
    comp.set_mem(PROGRAM_LOCATION + 6, 0x0E40);
    comp.set_mem(PROGRAM_LOCATION + 7, PROGRAM_LOCATION + 2);
    comp
    // todo!(
    //     "ADD #0x0001, &0x0010;
    //     JMP #PROGRAM_LOCATION;"
    // )
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
