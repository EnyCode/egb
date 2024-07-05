use alloc::boxed::Box;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point},
    pixelcolor::{Rgb565, RgbColor},
    text::Text,
    Drawable,
};

use crate::gui::{
    core::{NORMAL_TEXT, WHITE_CHAR},
    screen::Screen,
};

pub struct GamesScreen;

impl<D> Screen<D> for GamesScreen
where
    D: DrawTarget<Color = Rgb565> + OriginDimensions,
{
    fn draw(&mut self, display: &mut D) -> Result<(), D::Error> {
        display.clear(Rgb565::new(0, 1, 6))?;

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

        Text::with_text_style("Games", Point::new(10, 10), WHITE_CHAR, NORMAL_TEXT)
            .draw(display)?;

        Ok(())
    }

    fn update(&mut self, display: &mut D) -> Result<Option<Box<dyn Screen<D>>>, D::Error> {
        Ok(None)
    }
}
