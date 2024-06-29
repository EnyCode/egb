use std::collections::HashMap;
use std::iter::Map;
use std::str::FromStr;

use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::mono_font::mapping::StrGlyphMapping;
use embedded_graphics::mono_font::{DecorationDimensions, MonoFont};
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::primitives::{
    CornerRadii, Primitive, PrimitiveStyleBuilder, Rectangle, RoundedRectangle,
};
use embedded_graphics::text::{Alignment, Baseline, TextStyleBuilder};
use embedded_graphics::Drawable;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::MonoTextStyle,
    text::{Text, TextStyle},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use crate::games::Game;

const PICO_FONT: MonoFont = MonoFont {
    image: ImageRaw::new(include_bytes!("font.raw"), 128),
    glyph_mapping: &StrGlyphMapping::new(
        "  ! \" # $ % & ' ( ) * + , - . / 0 1 2 3 4 5 6 7 8 9 : ; < = > ? @ A B C D E F G H I J K L M N O P Q R S T U V W X Y Z [ \\ ] ^ _ ` a b c d e f g h i j k l m n o p q r s t u v w x y z { | } ~ \u{80} \u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8A}\u{8B}\u{8C}\u{8D}\u{8E}\u{8F}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9A}\u{9B}\u{9C}\u{9D}\u{9E}\u{9F}\u{A0}\u{A1}\u{A2}\u{A3}\u{A4}\u{A5}\u{A6}\u{A7}\u{A8}\u{A9}\u{AA}\u{AB}\u{AC}\u{AD}\u{AE}\u{AF}\u{B0}\u{B1}\u{B2}\u{B3}\u{B4}\u{B5}\u{B6}\u{B7}\u{B8}\u{B9}\u{BA}\u{BB}\u{BC}",
        0,
    ),
    character_size: Size::new(4, 6),
    character_spacing: 0,
    baseline: 0,
    underline: DecorationDimensions::default_underline(6),
    strikethrough: DecorationDimensions::default_strikethrough(3),
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

impl Into<String> for Button {
    fn into(self) -> String {
        match self {
            Button::Up => "\u{A9}\u{AA}",
            Button::Down => "\u{87}\u{88}",
            Button::Left => "\u{97}\u{98}",
            Button::Right => "\u{A3}\u{A4}",
            Button::A => "\u{B5}\u{B6}",
            Button::B => "\u{B7}\u{B8}",
            Button::Start => "\u{B9}\u{BA}",
            Button::Select => "\u{BB}\u{BC}",
        }
        .to_owned()
    }
}

pub struct GUI<D>
where
    D: DrawTarget<Color = Rgb565>,
{
    // TODO: move some text styles over if only used once
    white_char: MonoTextStyle<'static, Rgb565>,
    subtitle_char: MonoTextStyle<'static, Rgb565>,
    normal_text: TextStyle,
    centered_text: TextStyle,
    display: D,
    size: Size,
    // TODO: move to a global config type thing
    games: Vec<Game>,
    selected_game: u32,
}

impl GUI<SimulatorDisplay<Rgb565>> {
    pub fn update(&self) {
        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::Default)
            .pixel_spacing(0)
            .scale(3)
            .build();
        Window::new("Eny's GameBoy", &output_settings).show_static(&self.display);
    }
}

impl<D> GUI<D>
where
    D: DrawTarget<Color = Rgb565>,
{
    pub fn new(display: D, games: Vec<Game>) -> Self {
        let white_char = MonoTextStyle::new(&PICO_FONT, Rgb565::WHITE);
        let subtitle_char = MonoTextStyle::new(&PICO_FONT, Rgb565::new(24, 49, 24));
        let normal_text = TextStyleBuilder::new()
            .baseline(Baseline::Alphabetic)
            .alignment(Alignment::Left)
            .build();
        let mut centered_text = normal_text.clone();
        centered_text.alignment = Alignment::Center;

        return GUI {
            white_char,
            subtitle_char,
            normal_text,
            centered_text,
            size: display.bounding_box().size,
            display,
            games,
            selected_game: 0,
        };
    }

    pub fn draw_background(&mut self) -> Result<(), D::Error> {
        // light gray
        let background = PrimitiveStyleBuilder::new()
            .stroke_width(0)
            .fill_color(Rgb565::new(24, 49, 24))
            .build();
        // dark gray
        let borders = PrimitiveStyleBuilder::new()
            .stroke_width(0)
            .fill_color(Rgb565::new(14, 29, 14))
            .build();
        // navy blue
        let content = PrimitiveStyleBuilder::new()
            .stroke_width(0)
            .fill_color(Rgb565::new(0, 1, 6))
            .build();

        Rectangle::new(Point::zero(), self.size)
            .into_styled(background)
            .draw(&mut self.display)?;

        Rectangle::new(Point::zero(), Size::new(self.size.width, 8))
            .into_styled(borders)
            .draw(&mut self.display)?;

        Rectangle::new(
            Point::new(0, (self.size.height - 10).try_into().unwrap()),
            Size::new(self.display.bounding_box().size.width, 10),
        )
        .into_styled(borders)
        .draw(&mut self.display)?;

        Rectangle::new(
            Point::new(0, 16),
            Size::new(self.size.width, self.size.height - 34),
        )
        .into_styled(content)
        .draw(&mut self.display)?;

        Text::with_text_style(
            "EGB v0.1",
            Point::new(self.size.width as i32 - 33, self.size.height as i32 - 8),
            self.subtitle_char,
            self.normal_text,
        )
        .draw(&mut self.display)?;

        let mut inputs = HashMap::new();
        inputs.insert(Button::Start, "Settings");
        inputs.insert(Button::A, "Launch");

        let _ = self.draw_inputs(inputs)?;
        self.draw_games();

        Ok(())
    }

    pub fn draw_inputs(&mut self, inputs: HashMap<Button, &str>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let string = inputs.iter().fold(String::new(), |acc, (button, text)| {
            format!(
                "{}{}{} ",
                acc,
                <Button as Into<String>>::into(*button),
                text
            )
        });

        Text::with_text_style(
            &string,
            Point::new((self.size.width / 2) as i32, self.size.height as i32 - 25),
            self.white_char,
            self.centered_text,
        )
        .draw(&mut self.display)?;

        Ok(())
    }

    pub fn draw_games(&mut self) -> Result<(), D::Error> {
        let game = &self.games[self.selected_game as usize];

        let cartridge = PrimitiveStyleBuilder::new()
            .stroke_width(0)
            .fill_color(Rgb565::new(24, 49, 24))
            .build();

        let (x, y) = (
            (self.size.width as i32 - 72) / 2,
            (self.size.height as i32 - 85) / 2,
        );

        Rectangle::new(Point::new(x, y), Size::new(67, 80))
            .into_styled(cartridge)
            .draw(&mut self.display)?;

        let tga = game.get_image();

        Image::new(&tga, Point::new(x + 5, y + 20)).draw(&mut self.display)?;

        Ok(())
    }
}
