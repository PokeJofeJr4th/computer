use crate::{CPU, Computer};

const PROGRAM_POINTER: u16 = 0x8000;

#[test]
fn test_add_values() {
    let mut comp = CPU::new();
    comp.set_mem(CPU::INSTRUCTION_PTR, PROGRAM_POINTER);

    comp.insert_data(
        PROGRAM_POINTER as usize,
        &[
            // MOV #0001, r0
            0x0110,
            // MOV #8000, &6000
            0x0F10,
            0x8000,
            0x6000,
            // YIELD
            CPU::YIELD_INSTRUCTION,
            // SWP &6000, r0
            0x0E20,
            0x6000,
            // YIELD
            CPU::YIELD_INSTRUCTION,
            // JEQ &6000 #001 #PROGRAM_POINTER
            0x4D31,
            0x6000,
            PROGRAM_POINTER,
            // YIELD
            CPU::YIELD_INSTRUCTION,
        ],
    );

    comp.until_yield();

    // make sure it loaded registers correctly
    assert_eq!(comp.get_mem(0x6000), 0x8000);
    assert_eq!(comp.get_mem(0x0000), 0x0001);

    comp.until_yield();

    // make sure it swapped registers correctly
    assert_eq!(comp.get_mem(0x6000), 0x0001);
    assert_eq!(comp.get_mem(0x0000), 0x8000);

    comp.until_yield();

    // make sure it executes a conditional jump correctly
    assert_eq!(comp.get_mem(0x6000), 0x8000);
    assert_eq!(comp.get_mem(0x0000), 0x0001);
}
