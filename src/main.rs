use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use games::Game;
use tinytga::Tga;

mod games;
mod gui;
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

    util::write_font();
    let display = SimulatorDisplay::<Rgb565>::new(Size::new(240, 160));
    let mut gui = gui::GUI::new(display, games);

    gui.draw_background()?;

    gui.update();

    Ok(())
}
