#![no_std]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

#[cfg(target_arch = "x86_64")]
extern crate std;

use core::panic;

use alloc::vec;
use alloc::vec::Vec;
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
    image::Image,
    pixelcolor::{Rgb565, RgbColor},
    Drawable,
};

use embedded_graphics::draw_target::DrawTarget;
//use panic_probe as _;

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
const DEVICE: Option<Sprig> = None;

#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    main();

    loop {}
}

fn main() {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 10240;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    #[cfg(target_arch = "x86_64")]
    let mut device = Simulator::init();
    #[cfg(target_arch = "arm")]
    let mut device = Sprig::init();

    let disp = device.display();

    let mut games: Vec<Game> = vec![];
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
    ));
    /*//Err::<(), i32>(0).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();*/
    let mut gui = Gui::new(disp, games);
    gui.draw_background().unwrap();
    //let image = Tga::from_slice(include_bytes!("assets/games/super_mario_land.tga")).unwrap();

    //let image: Image<_> = Image::new(&image, Point::new(34, 8));

    //image.draw(disp).unwrap();

    #[cfg(target_arch = "x86_64")]
    loop {
        device.update();
    }
}

#[cfg(target_arch = "arm")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use embedded_hal::digital::{OutputPin, StatefulOutputPin};
    use rp2040_hal::Clock;

    unsafe {
        let mut pac = rp2040_hal::pac::Peripherals::steal();
        let core = rp2040_hal::pac::CorePeripherals::steal();

        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

        let clocks = hal::clocks::init_clocks_and_plls(
            12_000_000u32,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
        let sio = hal::Sio::new(pac.SIO);

        let pins = hal::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        //let mut led_l = pins.gpio28.into_push_pull_output();
        let mut led_r = pins.gpio4.into_push_pull_output();

        led_r.set_high().unwrap();

        loop {
            led_r.toggle().unwrap();
            delay.delay_ms(500u32);
        }
    }
}

// End of file
