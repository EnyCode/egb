use crate::buffer::Buffer;

pub trait Emulator {
    fn new() -> Self;
    fn tick(&mut self, display: &mut Buffer);
}
