use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{draw_target::DrawTarget, geometry::OriginDimensions, pixelcolor::Rgb565};

use crate::{events::Event, input::InputStatus};

pub trait Screen<D>
where
    D: DrawTarget<Color = Rgb565> + OriginDimensions,
{
    fn draw(&mut self, display: &mut D) -> Result<(), D::Error>;
    // TODO: take input
    fn update(
        &mut self,
        display: &mut D,
        input: &InputStatus,
    ) -> Result<Option<Box<dyn Screen<D>>>, D::Error>;
    fn events(&mut self) -> Vec<Event>;
}
