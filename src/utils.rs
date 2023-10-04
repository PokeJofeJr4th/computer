use std::fmt::Debug;

pub fn print_and_ret<T: Debug>(t: T) -> T {
    println!("{t:?}");
    t
}

pub fn prepend_vec<T>(a: T, mut b: Vec<T>) -> Vec<T> {
    b.insert(0, a);
    b
}

pub fn add_vecs<T>(a: Vec<T>, mut b: Vec<T>) -> Vec<T> {
    b.reserve(a.len());
    for i in a {
        b.insert(0, i);
    }
    b
}
