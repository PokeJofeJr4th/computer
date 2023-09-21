use computer::Computer;

const PROGRAM_LOCATION: u16 = 0x8000;

fn prog() -> Computer {
    let mut comp = Computer::new();
    comp.set_mem(PROGRAM_LOCATION, 0x1111);
    comp.set_mem(PROGRAM_LOCATION + 1, 0x0F40);
    comp.set_mem(PROGRAM_LOCATION + 2, PROGRAM_LOCATION);
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

    loop {
        let register_value = comp.get_mem(0);
        if register_value > 1000 {
            break;
        }
        println!("{register_value}");
        comp.tick();
    }

    println!("{comp:?}");
}
