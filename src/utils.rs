use std::fmt::Debug;

pub fn print_and_ret<T: Debug>(t: T) -> T {
    println!("{t:?}");
    t
}
