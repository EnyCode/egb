use embedded_graphics::{
    image::ImageRaw,
    mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

const PICO_FONT: MonoFont = MonoFont {
    image: ImageRaw::new(include_bytes!("font.raw"), 128),
    glyph_mapping: &StrGlyphMapping::new(
        " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8A}\u{8B}\u{8C}\u{8D}\u{8E}\u{8F}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9A}",
        0,
    ),
    character_size: Size::new(8, 6),
    character_spacing: 0,
    baseline: 0,
    underline: DecorationDimensions::default_underline(6),
    strikethrough: DecorationDimensions::default_strikethrough(3),
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(240, 160));

    let character_style = MonoTextStyle::new(&PICO_FONT, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(Baseline::Alphabetic)
        .alignment(Alignment::Left)
        .build();

    Text::with_text_style(
        " !\"#$%&'()*+,-./\n0123456789:;<=>?\n@ABCDEFGHIJKLMNO\nPQRSTUVWXYZ[\\]^_\n`abcdefghijklmno\npqrstuvwxyz{|}~\u{80}\n\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8A}\u{8B}\u{8C}\u{8D}\u{8E}\u{8F}\u{90}\n\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9A}",
        display.bounding_box().top_left,
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
