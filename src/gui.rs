use embedded_graphics::geometry::Size;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::mono_font::mapping::StrGlyphMapping;
use embedded_graphics::mono_font::{DecorationDimensions, MonoFont};
use embedded_graphics::text::{Alignment, Baseline, TextStyleBuilder};
use embedded_graphics::Drawable;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    text::{Text, TextStyle},
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

pub struct GUI {
    pub character_style: MonoTextStyle<'static, BinaryColor>,
    pub text_style: TextStyle,
}

impl GUI {
    pub fn new() -> Self {
        let character_style = MonoTextStyle::new(&PICO_FONT, BinaryColor::On);
        let text_style = TextStyleBuilder::new()
            .baseline(Baseline::Alphabetic)
            .alignment(Alignment::Left)
            .build();
        return GUI {
            character_style,
            text_style,
        };
    }

    pub fn draw_background<D: DrawTarget<Color = BinaryColor>>(
        &self,
        display: &mut D,
    ) -> Result<(), D::Error> {
        Text::with_text_style(
            " !\"#$%&'()*+,-./\n0123456789:;<=>?\n@ABCDEFGHIJKLMNO\nPQRSTUVWXYZ[\\]^_\n`abcdefghijklmno\npqrstuvwxyz{|}~\u{80}\n\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8A}\u{8B}\u{8C}\u{8D}\u{8E}\u{8F}\u{90}\n\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9A}",
            display.bounding_box().top_left,
            self.character_style,
            self.text_style,
        )
        .draw(display)?;

        Ok(())
    }
}
