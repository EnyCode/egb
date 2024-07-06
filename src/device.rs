use alloc::boxed::Box;
use embedded_graphics::draw_target::DrawTarget;

use crate::{
    buffer::Buffer,
    gui::screen::Screen,
    input::{self, InputStatus},
};

pub trait Device<D: DrawTarget, B: DrawTarget> {
    fn init(screen: Box<dyn Screen<B>>) -> Self;
    fn display(&mut self) -> &mut D;
    fn set_backlight(&mut self, brightness: u16);
    fn set_led_l(&mut self, brightness: u16);
    fn set_led_r(&mut self, brightness: u16);
    fn delay_ms(&mut self, ms: u32);
    fn delay_us(&mut self, us: u32);
    fn update_input(&mut self, input: &mut InputStatus) -> InputStatus;
    // TODO: could this be merged into update_input?
    fn update(&mut self, input: &InputStatus);
    // TODO: add more stuff
}
