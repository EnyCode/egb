use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use games::Game;
use input::InputStatus;
use tinytga::Tga;

mod games;
mod gui;
mod input;
mod nes;
mod util;

fn main() -> Result<(), core::convert::Infallible> {
    let mut games = vec![];
    games.push(Game::new_gameboy_advanced(
        "Super Mario Advanced",
        Tga::from_slice(include_bytes!("assets/games/super_mario_advanced.tga")).unwrap(),
    ));
    games.push(Game::new_gameboy(
        "Super Mario Land",
        Tga::from_slice(include_bytes!("assets/games/super_mario_land.tga")).unwrap(),
    ));
    games.push(Game::new_nes(
        "Super Mario Bros",
        Tga::from_slice(include_bytes!("assets/games/super_mario_bros.tga")).unwrap(),
    ));

    util::write_font();
    let display = SimulatorDisplay::<Rgb565>::new(Size::new(240, 160));
    let mut gui = gui::GUI::new(display, games);

    gui.draw_background()?;
    gui.create_window();

    let mut input = InputStatus::default();

    'running: loop {
        gui.update();
        input.update();

        for event in gui.events().unwrap() {
            match event {
                SimulatorEvent::KeyDown {
                    keycode,
                    keymod,
                    repeat,
                } => {
                    input.key_down(keycode, repeat);
                }
                SimulatorEvent::KeyUp {
                    keycode,
                    keymod,
                    repeat,
                } => {
                    input.key_up(keycode);
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        gui.update_input(&input).unwrap();
    }

    Ok(())
}
