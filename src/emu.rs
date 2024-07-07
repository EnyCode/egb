use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565};

pub trait Emulator<D>
where
    D: DrawTarget<Color = Rgb565>,
{
    fn new(display: &mut D) -> Self;
    fn tick(&mut self, display: &mut D) -> Result<(), D::Error>;
}
