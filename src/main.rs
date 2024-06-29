use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

mod gui;
mod input;
mod util;

fn main() -> Result<(), core::convert::Infallible> {
    util::write_font();
    let display = SimulatorDisplay::<Rgb565>::new(Size::new(240, 160));
    let mut gui = gui::GUI::new(display);

    gui.draw_background()?;

    gui.update();

    Ok(())
}
