use std::fmt::Debug;

pub fn print_and_ret<T: Debug>(t: T) -> T {
    println!("{t:?}");
    t
}

pub fn add_vecs<T>(mut a: Vec<T>, b: impl IntoIterator<Item = T>) -> Vec<T> {
    a.extend(b);
    a
}
