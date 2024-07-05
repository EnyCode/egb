use alloc::boxed::Box;
use embedded_graphics::{draw_target::DrawTarget, geometry::OriginDimensions, pixelcolor::Rgb565};

pub trait Screen<D>
where
    D: DrawTarget<Color = Rgb565> + OriginDimensions,
{
    // TODO: could this be static?
    fn draw(&mut self, display: &mut D) -> Result<(), D::Error>;
    fn update(&mut self, display: &mut D) -> Result<Option<Box<dyn Screen<D>>>, D::Error>;
}
