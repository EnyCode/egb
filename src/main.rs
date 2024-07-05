#![no_std]
#![no_main]

use device::Device;
use embedded_graphics::{
    geometry::Point,
    image::{Image, ImageRaw, ImageRawLE},
    pixelcolor::{Rgb565, RgbColor},
    Drawable,
};
use rp2040::Sprig;
use rp2040_hal::{
    self as hal,
    fugit::RateExtU32,
    gpio::{
        bank0::{Gpio16, Gpio18, Gpio19},
        FunctionSpi, Pin, PullDown,
    },
};

use defmt_rtt as _;
use embedded_graphics::draw_target::DrawTarget;
use embedded_hal::digital::OutputPin;
use hal::entry;
use panic_probe as _;

use hal::{clocks::Clock, pac};
use st7735_lcd::{Orientation, ST7735};

mod device;
mod rp2040;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[entry]
fn main() -> ! {
    let mut sprig = Sprig::init();
    let disp = sprig.display();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();
    disp.set_offset(0, 25);

    let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("ferris.raw"), 86);

    let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));

    image.draw(disp).unwrap();
    //sprig.set_led_l(30000);
    //sprig.set_led_r(0);
    //sprig.wait(1000);
    //sprig.set_backlight(0);

    // Wait until the background and image have been rendered otherwise
    // the screen will show random pixels for a brief moment
    //lcd_led.set_high().unwrap();
    //l_led.set_high().unwrap();

    loop {
        for i in (0..=u16::MAX).skip(100) {
            sprig.delay_us(8);
            sprig.set_led_l(i);
            sprig.set_backlight(i);
        }

        for i in (0..=u16::MAX).rev().skip(100) {
            sprig.delay_us(8);
            sprig.set_led_l(i);
            sprig.set_backlight(i);
        }

        sprig.delay_ms(500);
    }
}

// End of file
