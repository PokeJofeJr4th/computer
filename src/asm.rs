#[macro_export]
macro_rules! program {
    () => {};
    ($instruction_ptr:ident $comp:ident @ JMP #$lit:expr; $($rest:tt)*) => {
        $comp.set_mem($instruction_ptr, Computer::JMP_LIT);
        $comp.set_mem($instruction_ptr + 1, $lit);
        $instruction_ptr += 2;
        program!($instruction_ptr $comp @ $($rest)*);
    };
    ($instruction_ptr:ident $comp:ident @ ADD #$lit:expr, &$dest:expr; $($rest:tt)*) => {
        $comp.set_mem($instruction_ptr, Computer::ADD_LIT_DEST);
        $comp.set_mem($instruction_ptr + 1, $lit);
        $comp.set_mem($instruction_ptr + 2, $dest);
        $instruction_ptr += 3;
        program!($instruction_ptr $comp @ $($rest)*);
    };
    ($ip:ident $comp:ident @) => {};
    ($ip:ident $comp:ident @ $($t:tt)+) => {
        panic!("Incomplete ASM Macro - {}", stringify!($($t)+));
    };
    ($($t:tt)*) => {{
        let mut instruction_ptr: u16 = 0x8000;
        let mut comp: Computer = Computer::new();
        program!(instruction_ptr comp @ $($t)*);
        comp
    }};
}
