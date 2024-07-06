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
    events::Event,
    games::{Game, GameConsole},
    gui::{
        core::{
            draw_inputs, BACKGROUND, BLACK_CHAR, CENTERED_TEXT, GREY_CHAR, INNER_BORDER,
            NORMAL_TEXT, OUTER_BORDER_CLR, WHITE_CHAR,
        },
        screen::Screen,
    },
    input::{Button, InputStatus},
};

const TOTAL_FRAMES: i32 = 4;

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
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::None => 0,
        }
    }
}

pub struct GamesScreen {
    pub games: Vec<Game>,
    // TODO: this could probably be a different type
    frame: i32,
    previous_game: u8,
    dir: Direction,
    // TODO: could be a bigger type but maybe not necessary
    selected_game: u8,
    events: Vec<Event>,
}

impl GamesScreen {
    pub fn new(games: Vec<Game>) -> Self {
        Self::with_game(games, 255)
    }

    pub fn with_game(games: Vec<Game>, selected_game: u8) -> Self {
        Self {
            games,
            frame: 0,
            dir: Direction::None,
            selected_game,
            previous_game: 0,
            events: vec![],
        }
    }
}

impl<D> Screen<D> for GamesScreen
where
    D: DrawTarget<Color = Rgb565> + OriginDimensions,
{
    fn draw(&mut self, display: &mut D) -> Result<(), D::Error> {
        display.clear(OUTER_BORDER_CLR)?;
        let size = display.size();

        Rectangle::new(Point::new(0, 8), Size::new(size.width, size.height - 18))
            .into_styled(INNER_BORDER)
            .draw(display)?;

        Rectangle::new(Point::new(0, 18), Size::new(size.width, size.height - 36))
            .into_styled(BACKGROUND)
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
        let mut dirty = true;
        let mut pressed = false;
        //std::println!("{:?} {:?}", input.left.pressed, input.left.timer);
        if self.frame == 0 {
            if input.left.should_trigger() {
                self.selected_game = max(self.selected_game.saturating_sub(1), 0);
                self.dir = Direction::Left;
                pressed = true;
            } else if input.right.should_trigger() {
                self.selected_game = min(self.selected_game + 1, self.games.len() as u8 - 1);
                self.dir = Direction::Right;
                pressed = true;
            }
            if input.b.should_trigger() {
                return Ok(Some(Box::new(
                    crate::gui::screens::settings::Settings::new(
                        self.games.clone(),
                        if self.selected_game == 0 {
                            255
                        } else {
                            self.selected_game
                        },
                    ),
                )));
            }

            if input.a.should_trigger() {
                self.events.push(Event::LaunchGame(GameConsole::NES));
            }
        }

        if self.frame == 0 && self.previous_game == self.selected_game {
            dirty = false;
        }
        if self.frame == TOTAL_FRAMES && !pressed {
            dirty = false;
        }

        if !dirty {
            self.frame = 0;
            self.previous_game = self.selected_game;
            self.dir = Direction::None;
            return Ok(None);
        }

        if self.selected_game == 255 {
            self.selected_game = 0;
        }

        self.frame += 1;
        let multiplier: f32 = self.frame as f32 / TOTAL_FRAMES as f32;
        //std::println!("{:?} {:?}", self.frame, multiplier);

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
                    let old = GameConsole::GameBoy
                        .get_pos(&size, (i as i32 + self.dir.get_offset()) as u8);
                    let current = GameConsole::GameBoy.get_pos(&size, i as u8);
                    let (diff_x, diff_y) = (current.x - old.x, current.y - old.y);
                    let (x, y) = (
                        (diff_x as f32 * multiplier) as i32 + old.x,
                        (diff_y as f32 * multiplier) as i32 + old.y,
                    );

                    let cartridge: Tga<Rgb565> = Tga::from_slice(GB_CARTRIDGE).unwrap();
                    Image::new(&cartridge, Point::new(x, y)).draw(display)?;

                    let tga = game.get_image();

                    Image::new(&tga, Point::new(x + 10, y + 26)).draw(display)?;
                }
                crate::games::GameConsole::GameBoyColor => todo!(),
                crate::games::GameConsole::GameBoyAdvanced => {
                    let old = GameConsole::GameBoyAdvanced
                        .get_pos(&size, (i as i32 + self.dir.get_offset()) as u8);
                    let current = GameConsole::GameBoyAdvanced.get_pos(&size, i as u8);
                    let (diff_x, diff_y) = (current.x - old.x, current.y - old.y);
                    let (x, y) = (
                        (diff_x as f32 * multiplier) as i32 + old.x,
                        (diff_y as f32 * multiplier) as i32 + old.y,
                    );

                    let cartridge: Tga<Rgb565> = Tga::from_slice(GBA_CARTRIDGE).unwrap();
                    Image::new(&cartridge, Point::new(x, y)).draw(display)?;

                    let tga = game.get_image();

                    Image::new(&tga, Point::new(x + 15, y + 14)).draw(display)?;
                }
                crate::games::GameConsole::NES => {
                    let old =
                        GameConsole::NES.get_pos(&size, (i as i32 + self.dir.get_offset()) as u8);
                    let current = GameConsole::NES.get_pos(&size, i as u8);
                    let (diff_x, diff_y) = (current.x - old.x, current.y - old.y);
                    let (x, y) = (
                        (diff_x as f32 * multiplier) as i32 + old.x,
                        (diff_y as f32 * multiplier) as i32 + old.y,
                    );

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
            inputs.push((Button::B, "Settings"));

            draw_inputs(inputs, display, WHITE_CHAR)?;
        }

        Ok(None)
    }

    fn events(&mut self) -> Vec<crate::events::Event> {
        self.events.drain(..).collect()
    }
}
