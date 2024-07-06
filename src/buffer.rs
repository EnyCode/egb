use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions, Point, Size},
    pixelcolor::{Rgb565, RgbColor},
    primitives::Rectangle,
};
use embedded_graphics_framebuf::FrameBuf;

pub struct Buffer {
    buffer: FrameBuf<Rgb565, [Rgb565; 160 * 128]>,
    pub dirty: bool,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            buffer: FrameBuf::new([Rgb565::BLACK; 160 * 128], 160, 128),
            dirty: true,
        }
    }

    pub fn data(&self) -> [Rgb565; 160 * 128] {
        self.buffer.data
    }
}

impl OriginDimensions for Buffer {
    fn size(&self) -> Size {
        Size {
            width: 160,
            height: 128,
        }
    }
}

impl DrawTarget for Buffer {
    type Color = Rgb565;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        self.dirty = true;
        self.buffer.draw_iter(pixels)
    }
}
