use crate::Computer;

const PROGRAM_POINTER: u16 = 0x8000;

#[test]
fn test_add_values() {
    let mut comp = Computer::new();
    comp.set_mem(Computer::INSTRUCTION_PTR, PROGRAM_POINTER);

    // MOV #0001, r0
    comp.set_mem(PROGRAM_POINTER, 0x0110);
    // MOV #8000, &6000
    comp.set_mem(PROGRAM_POINTER + 1, 0x0F10);
    comp.set_mem(PROGRAM_POINTER + 2, 0x8000);
    comp.set_mem(PROGRAM_POINTER + 3, 0x6000);
    // YIELD
    comp.set_mem(PROGRAM_POINTER + 4, Computer::YIELD_INSTRUCTION);

    comp.until_yield();

    // make sure it loaded registers correctly
    assert_eq!(comp.get_mem(0x6000), 0x8000);
    assert_eq!(comp.get_mem(0x0000), 0x0001);

    // SWP &6000, r0
    comp.set_mem(PROGRAM_POINTER + 5, 0x0E20);
    comp.set_mem(PROGRAM_POINTER + 6, 0x6000);
    // YIELD
    comp.set_mem(PROGRAM_POINTER + 7, Computer::YIELD_INSTRUCTION);

    comp.until_yield();

    // make sure it swapped registers correctly
    assert_eq!(comp.get_mem(0x6000), 0x0001);
    assert_eq!(comp.get_mem(0x0000), 0x8000);

    // JEQ &6000 #0001 #PROGRAM_POINTER
    comp.set_mem(PROGRAM_POINTER + 8, 0x4D31);
    comp.set_mem(PROGRAM_POINTER + 9, 0x6000);
    comp.set_mem(PROGRAM_POINTER + 10, PROGRAM_POINTER);
    // YIELD
    comp.set_mem(PROGRAM_POINTER + 10, Computer::YIELD_INSTRUCTION);

    comp.until_yield();

    // make sure it executes a conditional jump correctly
    assert_eq!(comp.get_mem(0x6000), 0x8000);
    assert_eq!(comp.get_mem(0x0000), 0x0001);
}
