#![no_std]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

#[cfg(target_arch = "x86_64")]
extern crate std;

use alloc::vec;
#[cfg(target_arch = "arm")]
use defmt_rtt as _;
use games::Game;
use gui::Gui;
#[cfg(target_arch = "arm")]
use hal::entry;
#[cfg(target_arch = "arm")]
use rp2040::Sprig;
#[cfg(target_arch = "arm")]
use rp2040_hal as hal;

extern crate alloc;
#[cfg(target_arch = "arm")]
use embedded_alloc::Heap;

#[cfg(target_arch = "arm")]
#[global_allocator]
static HEAP: Heap = Heap::empty();

use device::Device;
use embedded_graphics::{
    geometry::Point,
    image::{Image, ImageRaw, ImageRawLE},
    pixelcolor::{Rgb565, RgbColor},
    Drawable,
};

use embedded_graphics::draw_target::DrawTarget;
use panic_probe as _;

//use st7735_lcd::Orientation;

mod device;
#[cfg(target_arch = "arm")]
mod rp2040;
#[cfg(target_arch = "x86_64")]
mod simulator;
#[cfg(target_arch = "x86_64")]
use simulator::Simulator;
use tinytga::Tga;
mod games;
mod gui;
mod input;

#[cfg(target_arch = "arm")]
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    main();

    loop {}
}

fn main() {
    #[cfg(target_arch = "x86_64")]
    let mut device = Simulator::init();
    #[cfg(target_arch = "arm")]
    let mut device = Sprig::init();
    let disp = device.display();

    /*let mut games = vec![];
    games.push(Game::new_gameboy_advanced(
        "Super Mario Advanced",
        Tga::from_slice(include_bytes!("assets/games/super_mario_advanced.tga")).unwrap(),
    ));
    games.push(Game::new_gameboy(
        "Super Mario Land",
        Tga::from_slice(include_bytes!("assets/games/super_mario_land.tga")).unwrap(),
    ));
    games.push(Game::new_nes(
        "Super Mario Bros",
        Tga::from_slice(include_bytes!("assets/games/super_mario_bros.tga")).unwrap(),
    ));*/
    disp.clear(Rgb565::BLACK).unwrap();
    //let mut gui = Gui::new(disp, games);
    //gui.draw_background().unwrap();

    let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("ferris.raw"), 86);

    let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));

    image.draw(disp).unwrap();

    #[cfg(target_arch = "x86_64")]
    loop {
        device.update();
    }
}

// End of file
