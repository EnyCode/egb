use core::{
    cmp::{max, min},
    f32::consts::PI,
};

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

pub(crate) enum Direction {
    Left,
    Right,
    None,
}

impl Direction {
    pub fn get_offset(&self) -> i32 {
        match self {
            Direction::Left => 1,
            Direction::Right => -1,
            Direction::None => 0,
        }
    }
}

pub struct GamesScreen {
    pub games: Vec<Game>,
    // TODO: this could probably be a different type
    frame: usize,
    dir: Direction,
    // TODO: could be a bigger type but maybe not necessary
    selected_game: u8,
}

impl GamesScreen {
    pub fn new(games: Vec<Game>) -> Self {
        Self {
            games,
            frame: 0,
            dir: Direction::None,
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
            // TODO: use SUBTITLE_CHAR on simulator - bad contrast on sprig
            WHITE_CHAR,
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
        if self.frame > 0 {
            dirty = true;
        } else {
            if input.left.should_trigger() {
                //self.selected_game = max(self.selected_game.saturating_sub(1), 0);
                if max(self.selected_game.saturating_sub(1), 0) != self.selected_game {
                    dirty = true;
                    self.dir = Direction::Left;
                }
            } else if input.right.should_trigger() {
                //self.selected_game = min(self.selected_game + 1, self.games.len() as u8 - 1);
                if min(self.selected_game + 1, self.games.len() as u8 - 1) != self.selected_game {
                    dirty = true;
                    self.dir = Direction::Right;
                }
            }
        }
        //std::println!("selected_game: {}", self.selected_game);
        if !dirty {
            self.dir = Direction::None;
            //self.frame = 0;
            if self.frame == 0 {
                return Ok(None);
            }
        }

        self.frame += 1;
        // if done, set frame to 0 - last refresh
        if self.frame == 4 {
            self.frame = 0;
            //return Ok(None);
        }

        let mut offset = self.frame as f32 / 4. * self.dir.get_offset() as f32 * 105.;
        if self.frame == 0 {
            offset = 0.;
            /*self.selected_game = min(
                max(self.selected_game + self.dir.get_offset() as u8, 0),
                self.games.len() as u8 - 1,
            );*/
        }
        let size = display.size();
        //std::println!("frame: {} {}", self.frame, (offset * 82.) as i32);

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
            self.games[min(
                max(self.selected_game + self.dir.get_offset() as u8, 0),
                self.games.len() as u8 - 1,
            ) as usize]
                .title,
            Point::new(size.width as i32 / 2, 12),
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
                    x += offset as i32;

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
                    x += offset as i32;

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
                    let target_x = if i == 0 {
                        0 - 82 + 16
                    } else if i == 1 {
                        (size.width as i32 - 82) / 2
                    } else {
                        size.width as i32 - 16
                    };
                    x += offset as i32;

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
