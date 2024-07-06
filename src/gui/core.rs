use core::fmt::Pointer;

use alloc::{boxed::Box, format, string::String, vec::Vec};
use embedded_graphics::Drawable;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    image::ImageRaw,
    mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont, MonoTextStyle},
    pixelcolor::{Rgb565, RgbColor},
    text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder},
};

use crate::input::{Button, InputStatus};

use super::screen;

const PICO_FONT: MonoFont = MonoFont {
    image: ImageRaw::new(include_bytes!("../assets/font.raw"), 128),
    glyph_mapping: &StrGlyphMapping::new(
        "  ! \" # $ % & ' ( ) * + , - . / 0 1 2 3 4 5 6 7 8 9 : ; < = > ? @ A B C D E F G H I J K L M N O P Q R S T U V W X Y Z [ \\ ] ^ _ ` a b c d e f g h i j k l m n o p q r s t u v w x y z { | } ~ \u{80} \u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8A}\u{8B}\u{8C}\u{8D}\u{8E}\u{8F}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9A}\u{9B}\u{9C}\u{9D}\u{9E}\u{9F}\u{A0}\u{A1}\u{A2}\u{A3}\u{A4}\u{A5}\u{A6}\u{A7}\u{A8}\u{A9}\u{AA}\u{AB}\u{AC}\u{AD}\u{AE}\u{AF}\u{B0}\u{B1}\u{B2}\u{B3}\u{B4}\u{B5}\u{B6}\u{B7}\u{B8}\u{B9}\u{BA}\u{BB}\u{BC}",
        0,
    ),
    character_size: Size::new(4, 6),
    character_spacing: 0,
    baseline: 0,
    // TODO: double check this
    underline: DecorationDimensions::default_underline(6),
    strikethrough: DecorationDimensions::default_strikethrough(3),
};

pub const WHITE_CHAR: MonoTextStyle<Rgb565> = MonoTextStyle::new(&PICO_FONT, Rgb565::WHITE);
pub const GREY_CHAR: MonoTextStyle<Rgb565> =
    MonoTextStyle::new(&PICO_FONT, Rgb565::new(24, 49, 24));
// TODO: Q: is this necessary?
pub const NORMAL_TEXT: TextStyle = TextStyleBuilder::new()
    .baseline(Baseline::Alphabetic)
    .alignment(Alignment::Left)
    .build();
pub const CENTERED_TEXT: TextStyle = TextStyleBuilder::new()
    .baseline(Baseline::Middle)
    .alignment(Alignment::Center)
    .build();

pub struct Gui<D>
where
    D: DrawTarget<Color = Rgb565> + OriginDimensions,
{
    screen: Box<dyn screen::Screen<D>>,
    pub input: InputStatus,
}

impl<D> Gui<D>
where
    D: DrawTarget<Color = Rgb565> + OriginDimensions,
{
    pub fn new(screen: Box<dyn screen::Screen<D>>, display: &mut D) -> Result<Self, D::Error> {
        let mut slf = Self {
            screen,
            input: InputStatus::default(),
        };

        slf.screen.draw(display)?;

        Ok(slf)
    }

    fn change_screen(
        &mut self,
        screen: Box<dyn screen::Screen<D>>,
        display: &mut D,
    ) -> Result<(), D::Error> {
        self.screen = screen;
        //self.display.clear(Rgb565::BLACK)?;
        self.screen.draw(display);

        Ok(())
    }

    pub fn update(&mut self, input: &InputStatus, display: &mut D) -> Result<(), D::Error> {
        let result = self.screen.update(display, input)?;
        if result.is_some() {
            self.change_screen(result.unwrap(), display)?;
        }

        Ok(())
    }
}

pub fn draw_inputs<D>(inputs: Vec<(Button, &str)>, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565> + OriginDimensions,
{
    let string = inputs.iter().fold(String::new(), |acc, (button, text)| {
        format!(
            "{}{}{} ",
            acc,
            <Button as Into<String>>::into(*button),
            text
        )
    });

    let size = display.size();

    Text::with_text_style(
        &string,
        Point::new((size.width / 2) as i32, size.height as i32 - 25),
        WHITE_CHAR,
        CENTERED_TEXT,
    )
    .draw(display)?;

    Ok(())
}
