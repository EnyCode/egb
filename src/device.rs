use embedded_graphics::draw_target::DrawTarget;

pub trait Device<D: DrawTarget> {
    fn init() -> Self;
    fn display(&mut self) -> &mut D;
    fn set_backlight(&mut self, brightness: u16);
    fn set_led_l(&mut self, brightness: u16);
    fn set_led_r(&mut self, brightness: u16);
    fn delay_ms(&mut self, ms: u32);
    fn delay_us(&mut self, us: u32);
    // TODO: add more stuff
}
