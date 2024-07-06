use core::cmp::{max, min};

use alloc::{boxed::Box, string::ToString, vec, vec::Vec};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    image::Image,
    pixelcolor::{Rgb565, RgbColor},
    primitives::{Primitive, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::Text,
    Drawable,
};
use tinytga::Tga;

use crate::{
    device::Device,
    events::Event,
    games::{Game, GameConsole},
    gui::{
        core::{
            draw_inputs, BACKGROUND, BLACK_CHAR, CENTERED_TEXT, GREY_CHAR, INNER_BORDER,
            INNER_BORDER_CLR, NORMAL_TEXT, OUTER_BORDER_CLR, WHITE_CHAR,
        },
        screen::Screen,
    },
    input::{Button, InputStatus},
    rp2040::Sprig,
};

use super::games::GamesScreen;

const TOTAL_FRAMES: i32 = 4;

const SLIDER: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_width(1)
    .stroke_color(OUTER_BORDER_CLR)
    .build();

const SELECTED: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_width(1)
    .stroke_color(INNER_BORDER_CLR)
    .build();

const SLIDER_FILL: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_width(0)
    .fill_color(Rgb565::new(10, 20, 10))
    .build();

const SELECTED_FILL: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_width(0)
    .fill_color(Rgb565::new(18, 37, 18))
    .build();

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Selection {
    Brightness,
    None,
}

impl Selection {
    pub fn next(&self) -> Self {
        match self {
            Selection::Brightness => Selection::None,
            Selection::None => Selection::Brightness,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Selection::Brightness => Selection::None,
            Selection::None => Selection::Brightness,
        }
    }

    pub fn point(&self) -> Point {
        match self {
            Selection::Brightness => Point::new(4, 31),
            Selection::None => Point::new(0, 0),
        }
    }
}

// TODO: use type parameters
pub struct Settings {
    events: Vec<Event>,
    games: Vec<Game>,
    selected_game: u8,
    brightness: u16,
    selection: Selection,
}

impl Settings {
    pub fn new(games: Vec<Game>, selected_game: u8) -> Self {
        Self {
            events: vec![],
            games,
            selected_game,
            brightness: u16::MAX,
            selection: Selection::None,
        }
    }
}

impl<D> Screen<D> for Settings
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

        Text::with_text_style(
            "Settings",
            Point::new(size.width as i32 / 2, 12),
            BLACK_CHAR,
            CENTERED_TEXT,
        )
        .draw(display)?;

        // TODO: helper for text bc this code very repetitive
        Text::with_text_style("Brightness", Point::new(4, 21), WHITE_CHAR, NORMAL_TEXT)
            .draw(display)?;

        Rectangle::new(Point::new(4, 31), Size::new(size.width - 24, 6))
            .into_styled(SLIDER)
            .draw(display)?;

        Rectangle::new(
            Point::new(5, 32),
            Size::new(
                (((size.width - 26) as f32 / u16::MAX as f32) * self.brightness as f32) as u32,
                4,
            ),
        )
        .into_styled(SLIDER_FILL)
        .draw(display)?;

        //self.update(display, &InputStatus::default())?;

        Ok(())
    }

    fn update(
        &mut self,
        display: &mut D,
        input: &InputStatus,
    ) -> Result<Option<Box<dyn Screen<D>>>, D::Error> {
        let mut dirty = false;
        let mut selection_dirty = false;
        let mut prev = self.selection.clone();
        let mut change = 0;
        if input.down.should_trigger() {
            self.selection = self.selection.next();
            dirty = true;
            selection_dirty = true;
        }
        if input.up.should_trigger() {
            self.selection = self.selection.previous();
            dirty = true;
            selection_dirty = true;
        }
        if input.b.should_trigger() {
            return Ok(Some(Box::new(GamesScreen::with_game(
                self.games.clone(),
                self.selected_game,
            ))));
        }
        if input.left.should_trigger() {
            change = -(u16::MAX as i32) / 17;
            dirty = true;
        }
        if input.right.should_trigger() {
            change = u16::MAX as i32 / 17;
            dirty = true;
        }

        if !dirty {
            return Ok(None);
        }

        let size = display.size();
        //std::println!("brightness {:?}", self.brightness);
        if selection_dirty {
            if prev != Selection::None {
                Rectangle::new(prev.point(), Size::new(size.width - 24, 6))
                    .into_styled(SLIDER)
                    .draw(display)?;

                let value = match prev {
                    Selection::Brightness => self.brightness,
                    Selection::None => panic!(),
                };
                let coord = prev.point();

                Rectangle::new(
                    Point::new(coord.x + 1, coord.y + 1),
                    Size::new(
                        (((size.width - 26) as f32 / u16::MAX as f32) * value as f32) as u32,
                        4,
                    ),
                )
                .into_styled(SLIDER_FILL)
                .draw(display)?;
            }
            if self.selection != Selection::None {
                Rectangle::new(self.selection.point(), Size::new(size.width - 24, 6))
                    .into_styled(SELECTED)
                    .draw(display)?;

                let value = match self.selection {
                    Selection::Brightness => self.brightness,
                    Selection::None => panic!(),
                };
                let coord = self.selection.point();

                Rectangle::new(
                    Point::new(coord.x + 1, coord.y + 1),
                    Size::new(
                        (((size.width - 26) as f32 / u16::MAX as f32) * value as f32) as u32,
                        4,
                    ),
                )
                .into_styled(SELECTED_FILL)
                .draw(display)?;
            }
        } else {
            if change != 0 && self.selection != Selection::None {
                match self.selection {
                    Selection::Brightness => {
                        self.brightness = if change > 0 {
                            self.brightness.saturating_add(change.abs() as u16)
                        } else {
                            self.brightness.saturating_sub(change.abs() as u16)
                        };
                        self.events
                            .push(Event::BacklightBrightness(self.brightness));
                    }
                    Selection::None => panic!(),
                }
                let coord = self.selection.point();
                if change < 0 {
                    Rectangle::new(
                        Point::new(coord.x + 1, coord.y + 1),
                        Size::new(size.width - 26, 4),
                    )
                    .into_styled(BACKGROUND)
                    .draw(display)?;
                }

                Rectangle::new(
                    Point::new(coord.x + 1, coord.y + 1),
                    Size::new(
                        (((size.width - 26) as f32 / u16::MAX as f32) * self.brightness as f32)
                            as u32,
                        4,
                    ),
                )
                .into_styled(SELECTED_FILL)
                .draw(display)?;
            }
        }

        Ok(None)
    }

    fn events(&mut self) -> Vec<crate::events::Event> {
        self.events.drain(..).collect()
    }
}
