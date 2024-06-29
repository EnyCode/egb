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
    let mut gui = gui::GUI::new(display.clone());

    gui.draw_background()?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Default)
        .pixel_spacing(0)
        .scale(3)
        .build();

    Window::new("Eny's GameBoy", &output_settings).show_static(&display);

    Ok(())
}
