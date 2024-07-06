use crate::gui::core::Gui;
use crate::gui::screen::Screen;
use embedded_graphics::{geometry::Size, pixelcolor::Rgb565};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettings, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use std::boxed::Box;

type Display = SimulatorDisplay<Rgb565>;

use crate::input::InputStatus;
use crate::Device;
use core::time::Duration;
use std::thread;

pub struct Simulator {
    display: Display,
    window: Window,
    gui: Option<Gui<Display>>,
}

impl Device<Display> for Simulator {
    fn init(screen: Box<dyn Screen<Display>>) -> Self {
        let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(160, 128));
        let settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::Default)
            .pixel_spacing(0)
            .build();
        let window = Window::new("EGB Simulator", &settings);
        Self {
            gui: Some(Gui::new(screen, &mut display).unwrap()),
            display,
            window,
        }
    }
    fn display(&mut self) -> &mut Display {
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

    fn update_input(&mut self, input: &mut InputStatus) -> InputStatus {
        let mut new = InputStatus::default();
        new
    }

    fn update(&mut self, input: &InputStatus) {
        //self.window.update(&self.display);
    }
}

impl Simulator {
    pub fn update(&mut self) {
        self.window.update(&self.display);
    }

    pub fn show_static(&mut self) {
        self.window.show_static(&self.display);
    }
}
