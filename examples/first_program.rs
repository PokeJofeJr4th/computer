use computer::{Computer, CPU};

const PROGRAM_LOCATION: u16 = 0x8000;

fn prog() -> CPU {
    let mut comp = CPU::new();
    comp.insert_data(PROGRAM_LOCATION, &[0x1111, 0x0F40, PROGRAM_LOCATION]);
    comp
}

fn main() {
    let mut comp = prog();

    // start running the program
    comp.set_mem(CPU::INSTRUCTION_PTR, PROGRAM_LOCATION);
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
