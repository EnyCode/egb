use core::cmp::{max, min};

use alloc::{boxed::Box, string::ToString, vec, vec::Vec};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    image::Image,
    pixelcolor::{Rgb565, RgbColor},
    primitives::{Primitive, PrimitiveStyleBuilder, Rectangle},
    text::Text,
    Drawable,
};
use tinytga::Tga;

use crate::{
    games::Game,
    gui::{
        core::{draw_inputs, BLACK_CHAR, CENTERED_TEXT, GREY_CHAR, NORMAL_TEXT, WHITE_CHAR},
        screen::Screen,
    },
    input::{Button, InputStatus},
};

// TODO: could these be moved to the sd card to save space for the emulators?
const GB_CARTRIDGE: &'static [u8; 4193] = include_bytes!("../../assets/cartridges/gb.tga");
const GBA_CARTRIDGE: &'static [u8; 3813] = include_bytes!("../../assets/cartridges/gba.tga");
const NES_CARTRIDGE: &'static [u8; 4709] = include_bytes!("../../assets/cartridges/nes.tga");

pub struct GamesScreen {
    pub games: Vec<Game>,
    // TODO: this could probably be a different type
    frame: usize,
    // TODO: could be a bigger type but maybe not necessary
    selected_game: u8,
}

impl GamesScreen {
    pub fn new(games: Vec<Game>) -> Self {
        Self {
            games,
            frame: 0,
            selected_game: 0,
        }
    }
}

impl<D> Screen<D> for GamesScreen
where
    D: DrawTarget<Color = Rgb565> + OriginDimensions,
{
    fn draw(&mut self, display: &mut D) -> Result<(), D::Error> {
        display.clear(Rgb565::new(14, 29, 14))?;
        let size = display.size();

        let inner_border = PrimitiveStyleBuilder::new()
            .stroke_width(0)
            .fill_color(Rgb565::new(24, 49, 24))
            .build();
        let background = PrimitiveStyleBuilder::new()
            .stroke_width(0)
            .fill_color(Rgb565::new(0, 1, 6))
            .build();

        Rectangle::new(Point::new(0, 8), Size::new(size.width, size.height - 18))
            .into_styled(inner_border)
            .draw(display)?;

        Rectangle::new(Point::new(0, 18), Size::new(size.width, size.height - 36))
            .into_styled(background)
            .draw(display)?;

        Text::with_text_style(
            &("EGB v".to_string() + env!("CARGO_PKG_VERSION")),
            Point::new(size.width as i32 - 41, size.height as i32 - 8),
            GREY_CHAR,
            NORMAL_TEXT,
        )
        .draw(display)?;

        // draw games
        self.update(display, &InputStatus::default())?;

        Ok(())
    }

    fn update(
        &mut self,
        display: &mut D,
        input: &InputStatus,
    ) -> Result<Option<Box<dyn Screen<D>>>, D::Error> {
        let mut dirty = false;
        //std::println!("{:?} {:?}", input.left.pressed, input.left.timer);
        if input.left.should_trigger() {
            self.selected_game = max(self.selected_game.saturating_sub(1), 0);
            dirty = true;
        } else if input.right.should_trigger() {
            self.selected_game = min(self.selected_game + 1, self.games.len() as u8 - 1);
            dirty = true;
        }
        //std::println!("selected_game: {}", self.selected_game);
        if !dirty {
            return Ok(None);
        }

        let size = display.size();

        let background = PrimitiveStyleBuilder::new()
            .stroke_width(0)
            .fill_color(Rgb565::new(0, 1, 6))
            .build();

        let inner_border = PrimitiveStyleBuilder::new()
            .stroke_width(0)
            .fill_color(Rgb565::new(24, 49, 24))
            .build();

        Rectangle::new(Point::new(0, 18), Size::new(size.width, size.height - 36))
            .into_styled(background)
            .draw(display)?;

        Rectangle::new(Point::new(0, 8), Size::new(size.width, 10))
            .into_styled(inner_border)
            .draw(display)?;

        Text::with_text_style(
            self.games[self.selected_game as usize].title,
            Point::new(size.width as i32 / 2, 10),
            BLACK_CHAR,
            CENTERED_TEXT,
        )
        .draw(display)?;

        let mut to_draw = vec![];
        let placeholder = Game::new_placeholder();
        if self.selected_game == 0 {
            to_draw.push(&placeholder);
        }

        for (i, game) in self.games.iter().enumerate() {
            let i = i as i32;
            if i + 1 == self.selected_game.into()
                || i == self.selected_game.into()
                || i - 1 == self.selected_game.into()
            {
                to_draw.push(game);
            }
        }

        /*let mut out = alloc::string::String::new();
        out += "Selected: ";
        for (i, game) in to_draw.iter().enumerate() {
            out += &i.to_string();
            out += ". ";
            out += game.title;
            out += ", ";
        }
        std::println!("{:?}", &out);*/

        for (i, game) in to_draw.iter().enumerate() {
            match game.get_console() {
                crate::games::GameConsole::GameBoy => {
                    // Left
                    let (mut x, mut y) = ((0 - 82) + 16, (size.height as i32 - 91) / 2);
                    // Center
                    if i == 1 {
                        (x, y) = ((size.width as i32 - 82) / 2, (size.height as i32 - 91) / 2);
                    // Right
                    } else if i == 2 {
                        (x, y) = (size.width as i32 - 16, (size.height as i32 - 91) / 2);
                    }

                    let cartridge: Tga<Rgb565> = Tga::from_slice(GB_CARTRIDGE).unwrap();
                    Image::new(&cartridge, Point::new(x, y)).draw(display)?;

                    let tga = game.get_image();

                    Image::new(&tga, Point::new(x + 10, y + 26)).draw(display)?;
                }
                crate::games::GameConsole::GameBoyColor => todo!(),
                crate::games::GameConsole::GameBoyAdvanced => {
                    // TODO: i feel like this code code be shorter
                    let (mut x, mut y) = ((0 - 106) + 16, (size.height as i32 - 61) / 2);
                    if i == 1 {
                        (x, y) = ((size.width as i32 - 106) / 2, (size.height as i32 - 61) / 2);
                    } else if i == 2 {
                        (x, y) = (size.width as i32 - 16, (size.height as i32 - 61) / 2);
                    }

                    let cartridge: Tga<Rgb565> = Tga::from_slice(GBA_CARTRIDGE).unwrap();
                    Image::new(&cartridge, Point::new(x, y)).draw(display)?;

                    let tga = game.get_image();

                    Image::new(&tga, Point::new(x + 15, y + 14)).draw(display)?;
                }
                crate::games::GameConsole::NES => {
                    let (mut x, mut y) = ((0 - 82) + 16, (size.height as i32 - 91) / 2);
                    if i == 1 {
                        (x, y) = ((size.width as i32 - 82) / 2, (size.height as i32 - 91) / 2);
                    } else if i == 2 {
                        (x, y) = (size.width as i32 - 16, (size.height as i32 - 91) / 2);
                    }

                    let cartridge: Tga<Rgb565> = Tga::from_slice(NES_CARTRIDGE).unwrap();
                    Image::new(&cartridge, Point::new(x, y)).draw(display)?;

                    let tga = game.get_image();

                    Image::new(&tga, Point::new(x + 38, y + 1)).draw(display)?;
                }
                crate::games::GameConsole::Sprig => todo!(),
                crate::games::GameConsole::Placeholder => (),
            }
            let mut inputs = vec![];
            inputs.push((Button::A, "Launch"));

            draw_inputs(inputs, display, WHITE_CHAR)?;
        }

        Ok(None)
    }
}
