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
        core::{draw_inputs, BLACK_CHAR, CENTERED_TEXT, GREY_CHAR, NORMAL_TEXT, WHITE_CHAR},
        screen::Screen,
    },
    input::{Button, InputStatus},
    rp2040::Sprig,
};

use super::games::GamesScreen;

const TOTAL_FRAMES: i32 = 4;

const SLIDER: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_width(1)
    .stroke_color(Rgb565::new(14, 29, 14))
    .build();

const SELECTED: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_width(1)
    .stroke_color(Rgb565::new(24, 49, 24))
    .build();

const SLIDER_FILL: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_width(0)
    .fill_color(Rgb565::new(10, 20, 10))
    .build();

#[derive(Debug, Clone)]
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
            brightness: u16::MAX / 10 * 9,
            selection: Selection::None,
        }
    }
}

impl<D> Screen<D> for Settings
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
                (((size.width - 24) as f32 / u16::MAX as f32) * self.brightness as f32) as u32,
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

        let size = display.size();

        if selection_dirty {
            Rectangle::new(prev.point(), Size::new(size.width - 24, 6))
                .into_styled(SLIDER)
                .draw(display)?;

            Rectangle::new(self.selection.point(), Size::new(size.width - 24, 6))
                .into_styled(SELECTED)
                .draw(display)?;
        }

        if !dirty {
            return Ok(None);
        }

        Ok(None)
    }

    fn events(&mut self) -> Vec<crate::events::Event> {
        todo!()
    }
}
