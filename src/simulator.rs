use crate::emu::Emulator;
use crate::events::Event;
use crate::games::GameConsole;
use crate::gui::core::Gui;
use crate::gui::screen::Screen;
use crate::nes::emu::NesEmulator;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::{geometry::Size, pixelcolor::Rgb565};
use embedded_graphics_simulator::sdl2::Keycode;
use embedded_graphics_simulator::SimulatorEvent;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettings, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use std::boxed::Box;
type Display = SimulatorDisplay<Rgb565>;

use crate::input::InputStatus;
use crate::Device;
use core::time::Duration;
use std::{println, thread};

pub struct Simulator {
    display: Display,
    window: Window,
    gui: Option<Gui<Display>>,
    nes_emu: Option<NesEmulator>,
}

impl Device<Display, Display> for Simulator {
    fn init(screen: Box<dyn Screen<Display>>) -> Self {
        let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(160, 128));
        let settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::Default)
            .pixel_spacing(0)
            .build();
        let mut window = Window::new("EGB Simulator", &settings);
        window.update(&display);
        Self {
            gui: Some(Gui::new(screen, &mut display).unwrap()),
            display,
            window,
            nes_emu: None,
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
        let mut new = input.clone();

        for event in self.window.events() {
            match event {
                SimulatorEvent::Quit => panic!("Exiting emulator *gracefully*"),
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::W => input.up.pressed = true,
                    Keycode::A => input.left.pressed = true,
                    Keycode::S => input.down.pressed = true,
                    Keycode::D => input.right.pressed = true,
                    Keycode::E => input.a.pressed = true,
                    Keycode::R => input.b.pressed = true,
                    _ => {}
                },
                SimulatorEvent::KeyUp { keycode, .. } => match keycode {
                    Keycode::W => input.up.pressed = false,
                    Keycode::A => input.left.pressed = false,
                    Keycode::S => input.down.pressed = false,
                    Keycode::D => input.right.pressed = false,
                    Keycode::E => input.a.pressed = false,
                    Keycode::R => input.b.pressed = false,
                    _ => {}
                },
                _ => {}
            }
        }
        new.update(
            input.up.pressed,
            input.down.pressed,
            input.left.pressed,
            input.right.pressed,
            input.a.pressed,
            input.b.pressed,
        );

        new
    }

    fn update(&mut self, input: &InputStatus) {
        //self.window.update(&self.display);
        if self.gui.is_some() {
            self.gui
                .as_mut()
                .unwrap()
                .update(input, &mut self.display)
                .unwrap();

            let events = self.gui.as_mut().unwrap().events();
            for event in events {
                match event {
                    Event::LaunchGame(console) => self.launch(console),
                    _ => (),
                }
            }
        } else if self.nes_emu.is_some() {
            self.nes_emu
                .as_mut()
                .unwrap()
                .tick(&mut self.display)
                .unwrap();
        }
    }
}

impl Simulator {
    pub fn update_window(&mut self) {
        self.window.update(&self.display);
    }

    pub fn show_static(&mut self) {
        self.window.show_static(&self.display);
    }

    fn launch(&mut self, console: GameConsole) {
        self.display.clear(Rgb565::BLACK);
        self.gui = None;
        self.nes_emu = Some(NesEmulator::new(&mut self.display));
    }
}
