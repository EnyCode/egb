use core::cmp::min;

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
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
use tinytga::Tga;

use crate::games::{Game, GameConsole};
use crate::input::Button;

const PICO_FONT: MonoFont = MonoFont {
    image: ImageRaw::new(include_bytes!("assets/font.raw"), 128),
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

const GB_CARTRIDGE: &'static [u8; 4193] = include_bytes!("assets/cartridges/gb.tga");
const GBA_CARTRIDGE: &'static [u8; 3813] = include_bytes!("assets/cartridges/gba.tga");
const NES_CARTRIDGE: &'static [u8; 4709] = include_bytes!("assets/cartridges/nes.tga");

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

pub struct Gui<'a, D>
where
    D: DrawTarget<Color = Rgb565>,
{
    // TODO: move some text styles over if only used once
    white_char: MonoTextStyle<'static, Rgb565>,
    subtitle_char: MonoTextStyle<'static, Rgb565>,
    normal_text: TextStyle,
    centered_text: TextStyle,
    display: &'a mut D,
    size: Size,
    // TODO: move to a global config type thing
    games: Vec<Game>,
    selected_game: u32,
}

#[cfg(feature = "simulator")]
impl Gui<SimulatorDisplay<Rgb565>> {
    pub fn create_window(&mut self) {
        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::Default)
            .pixel_spacing(0)
            .scale(3)
            .build();
        self.window = Some(Window::new("Eny's GameBoy", &output_settings))
    }

    pub fn update(&mut self) {
        if self.window.is_some() {
            self.window.as_mut().unwrap().update(&mut self.display);
        }
    }

    pub fn events(&mut self) -> Option<impl Iterator<Item = SimulatorEvent> + '_> {
        if self.window.is_some() {
            let events = (&mut self.window).as_mut().unwrap().events();
            return Some(events);
        }
        None
    }
}

impl<'a, D> Gui<'a, D>
where
    D: DrawTarget<Color = Rgb565>,
{
    pub fn new(display: &'a mut D, games: Vec<Game>) -> Self {
        let white_char = MonoTextStyle::new(&PICO_FONT, Rgb565::WHITE);
        let subtitle_char = MonoTextStyle::new(&PICO_FONT, Rgb565::new(24, 49, 24));
        let normal_text = TextStyleBuilder::new()
            .baseline(Baseline::Alphabetic)
            .alignment(Alignment::Left)
            .build();
        let mut centered_text = normal_text.clone();
        centered_text.alignment = Alignment::Center;

        return Gui {
            white_char,
            subtitle_char,
            normal_text,
            centered_text,
            size: display.bounding_box().size,
            display,
            games,
            selected_game: 1,
            #[cfg(feature = "simulator")]
            window: None,
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
            .draw(self.display)?;

        Rectangle::new(Point::zero(), Size::new(self.size.width, 8))
            .into_styled(borders)
            .draw(self.display)?;

        Rectangle::new(
            Point::new(0, (self.size.height - 10).try_into().unwrap()),
            Size::new(self.display.bounding_box().size.width, 10),
        )
        .into_styled(borders)
        .draw(self.display)?;

        Rectangle::new(
            Point::new(0, 16),
            Size::new(self.size.width, self.size.height - 34),
        )
        .into_styled(content)
        .draw(self.display)?;

        Text::with_text_style(
            "EGB v0.1",
            Point::new(self.size.width as i32 - 33, self.size.height as i32 - 8),
            self.subtitle_char,
            self.normal_text,
        )
        .draw(self.display)?;

        let mut inputs = Vec::new();
        inputs.push((Button::Start, "Settings"));
        inputs.push((Button::A, "Launch"));

        let _ = self.draw_inputs(inputs)?;
        self.draw_games()?;

        Ok(())
    }

    pub fn draw_inputs(&mut self, inputs: Vec<(Button, &str)>) -> Result<(), D::Error>
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
        .draw(self.display)?;

        Ok(())
    }

    pub fn draw_games(&mut self) -> Result<(), D::Error> {
        let mut to_draw = vec![];
        let game = Game::new_placeholder();

        if self.selected_game > 0 {
            to_draw.push(&self.games[self.selected_game as usize - 1]);
        } else {
            to_draw.push(&game);
        }
        to_draw.push(&self.games[self.selected_game as usize]);
        if self.selected_game < self.games.len() as u32 - 1 {
            to_draw.push(&self.games[self.selected_game as usize + 1]);
        }
        for (i, game) in to_draw.into_iter().enumerate() {
            match game.get_console() {
                GameConsole::GameBoy => {
                    let (mut x, mut y) = ((0 - 100) / 2, (self.size.height as i32 - 70) / 2);
                    if i == 1 {
                        (x, y) = (
                            (self.size.width as i32 - 82) / 2,
                            (self.size.height as i32 - 91) / 2,
                        );
                    } else if i == 2 {
                        (x, y) = (
                            self.size.width as i32 - 50,
                            (self.size.height as i32 - 70) / 2,
                        );
                    }

                    let cartridge: Tga<Rgb565> = Tga::from_slice(GB_CARTRIDGE).unwrap();
                    Image::new(&cartridge, Point::new(x, y)).draw(self.display)?;

                    let tga = game.get_image();

                    Image::new(&tga, Point::new(x + 10, y + 26)).draw(self.display)?;
                }
                GameConsole::GameBoyColor => todo!(),
                GameConsole::GameBoyAdvanced => {
                    let (mut x, mut y) = ((0 - 106) / 2, (self.size.height as i32 - 70) / 2);
                    if i == 1 {
                        (x, y) = (
                            (self.size.width as i32 - 106) / 2,
                            (self.size.height as i32 - 61) / 2,
                        );
                    } else if i == 2 {
                        (x, y) = (
                            self.size.width as i32 - 50,
                            (self.size.height as i32 - 70) / 2,
                        );
                    }

                    let cartridge: Tga<Rgb565> = Tga::from_slice(GBA_CARTRIDGE).unwrap();
                    Image::new(&cartridge, Point::new(x, y)).draw(self.display)?;

                    let tga = game.get_image();

                    Image::new(&tga, Point::new(x + 15, y + 14)).draw(self.display)?;
                }
                GameConsole::NES => {
                    let (mut x, mut y) = ((0 - 100) / 2, (self.size.height as i32 - 70) / 2);
                    if i == 1 {
                        (x, y) = (
                            (self.size.width as i32 - 82) / 2,
                            (self.size.height as i32 - 91) / 2,
                        );
                    } else if i == 2 {
                        (x, y) = (
                            self.size.width as i32 - 50,
                            (self.size.height as i32 - 70) / 2,
                        );
                    }

                    let cartridge: Tga<Rgb565> = Tga::from_slice(NES_CARTRIDGE).unwrap();
                    Image::new(&cartridge, Point::new(x, y)).draw(self.display)?;

                    let tga = game.get_image();

                    Image::new(&tga, Point::new(x + 38, y + 1)).draw(self.display)?;
                }
                GameConsole::Sprig => todo!(),
                _ => {}
            }
        }

        Ok(())
    }
}
