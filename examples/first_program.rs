use computer::Computer;

const PROGRAM_LOCATION: u16 = 0x8000;

fn prog() -> Computer {
    todo!(
        "ADD #0x0001, &0x0010;
        JMP #PROGRAM_LOCATION;"
    )
}

fn main() {
    let mut comp = prog();

    // start running the program
    comp.set_mem(Computer::INSTRUCTION_PTR, PROGRAM_LOCATION);
    println!("{comp:?}");

    loop {
        let register_value = comp.get_mem(0x0010);
        if register_value > 1000 {
            break;
        }
        println!("{register_value}");
        comp.tick();
    }
}
