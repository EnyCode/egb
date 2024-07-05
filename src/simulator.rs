use embedded_graphics::{geometry::Size, pixelcolor::Rgb565};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettings, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use crate::Device;
use core::time::Duration;
use std::thread;

pub struct Simulator {
    display: SimulatorDisplay<Rgb565>,
    window: Window,
}

impl Device<SimulatorDisplay<Rgb565>> for Simulator {
    fn init() -> Self {
        let display = SimulatorDisplay::<Rgb565>::new(Size::new(160, 128));
        let settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::Default)
            .build();
        let window = Window::new("EGB Simulator", &settings);
        Self { display, window }
    }
    fn display(&mut self) -> &mut SimulatorDisplay<Rgb565> {
        &mut self.display
    }
    fn set_backlight(&mut self, _brightness: u16) {
        return;
    }
    fn set_led_l(&mut self, _brightness: u16) {
        return;
    }
    fn set_led_r(&mut self, _brightness: u16) {
        return;
    }
    fn delay_ms(&mut self, ms: u32) {
        thread::sleep(Duration::from_millis(ms as u64));
    }
    fn delay_us(&mut self, us: u32) {
        thread::sleep(Duration::from_micros(us as u64));
    }
}

impl Simulator {
    pub fn update(&mut self) {
        self.window.update(&self.display);
    }
}
