use embedded_graphics::{
    image::ImageRaw,
    mono_font::{ascii::FONT_6X9, mapping::ASCII, DecorationDimensions, MonoFont, MonoTextStyle},
    pixelcolor::{BinaryColor, Gray8},
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
const PICO_FONT: MonoFont = MonoFont {
    image: ImageRaw::new(include_bytes!("font.png"), 128),
    glyph_mapping: &ASCII,
    character_size: Size::new(4, 7),
    character_spacing: 1,
    baseline: 7,
    underline: DecorationDimensions::default_underline(40),
    strikethrough: DecorationDimensions::default_strikethrough(40),
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(240, 160));

    let character_style = MonoTextStyle::new(&PICO_FONT, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(Baseline::Bottom)
        .alignment(Alignment::Center)
        .build();

    Text::with_text_style(
        "!",
        display.bounding_box().center(),
        character_style,
        text_style,
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Default)
        .pixel_spacing(0)
        .build();

    Window::new("Eny's GameBoy", &output_settings).show_static(&display);

    Ok(())
}
