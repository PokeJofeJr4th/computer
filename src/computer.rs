use std::fmt::Debug;

pub trait Computer {
    fn insert_data(&mut self, idx: impl Into<usize>, data: &[u16]);
    fn until_yield(&mut self);
    fn set_mem(&mut self, idx: u16, value: u16);
    fn get_mem(&self, idx: u16) -> u16;
}

#[allow(clippy::module_name_repetitions)]
pub trait ComputerDebug: Computer + Debug {
    fn debug_until_yield(&mut self);
}
