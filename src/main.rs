#![no_std]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

#[cfg(target_arch = "x86_64")]
extern crate std;

use core::panic;

use alloc::vec::Vec;
use alloc::{boxed::Box, vec};
#[cfg(target_arch = "arm")]
use defmt_rtt as _;
use games::Game;
use gui::{core::Gui, screens::games::GamesScreen};
#[cfg(target_arch = "arm")]
use hal::entry;
use input::InputStatus;
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
use simulator::Simulator;
use tinytga::Tga;
mod buffer;
mod emu;
mod events;
#[cfg(target_arch = "x86_64")]
mod font;
mod games;
mod gui;
mod input;
mod nes;

use rp2040::Sprig;

#[cfg(target_arch = "arm")]
mod simulator {
    pub struct Simulator;
}

#[cfg(target_arch = "x86_64")]
mod rp2040 {
    pub struct Sprig;
}

#[cfg(target_arch = "arm")]
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    use input::InputStatus;

    let mut sprig = shared().0.unwrap();
    let mut input = InputStatus::default();

    loop {
        input = sprig.update_input(&mut input);
        sprig.update(&input);
        sprig.delay_us(8);
    }
}

/// Shared code between main and entry
/// Contains info about the device and the games
fn shared() -> (Option<Sprig>, Option<Simulator>) {
    #[cfg(target_arch = "arm")]
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 10240;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut games: Vec<Game> = vec![];
    games.push(Game::new_nes(
        "Super Mario Bros",
        Tga::from_slice(include_bytes!("assets/games/super_mario_bros.tga")).unwrap(),
    ));
    games.push(Game::new_gameboy_advanced(
        "Super Mario Advanced",
        Tga::from_slice(include_bytes!("assets/games/super_mario_advanced.tga")).unwrap(),
    ));
    games.push(Game::new_gameboy(
        "Super Mario Land",
        Tga::from_slice(include_bytes!("assets/games/super_mario_land.tga")).unwrap(),
    ));

    // TODO: maybe add a startup screen?
    #[cfg(target_arch = "x86_64")]
    let mut device = Simulator::init(Box::new(GamesScreen::new(games)));
    #[cfg(target_arch = "arm")]
    let mut device = Sprig::init(Box::new(GamesScreen::new(games)));

    let disp = device.display();

    //#[cfg(target_arch = "x86_64")]
    //device.show_static();

    #[cfg(target_arch = "x86_64")]
    return (None, Some(device));
    #[cfg(target_arch = "arm")]
    return (Some(device), None);
}

#[cfg(target_arch = "x86_64")]
fn main() {
    //font::write_font();

    let mut sim = shared().1.unwrap();
    let mut input = InputStatus::default();
    let mut val = 0;

    loop {
        input = sim.update_input(&mut input);
        sim.update(&input);
        if val % 500 == 0 {
            sim.update_window();
        }
        //sim.delay_ms(100);
        val += 1;
    }
}

// TODO: move to seperate file & add blinking error codes
#[cfg(target_arch = "arm")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use alloc::{borrow::ToOwned, format, string::String};
    use embedded_hal::digital::{OutputPin, StatefulOutputPin};
    use rp2040_hal::{usb::UsbBus, Clock};
    use usb_device::{
        bus::UsbBusAllocator,
        device::{StringDescriptors, UsbDeviceBuilder, UsbVidPid},
    };
    use usbd_serial::SerialPort;

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

        let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

        let usb_bus = UsbBusAllocator::new(UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        ));

        let mut serial = SerialPort::new(&usb_bus);

        let payload = info.payload();
        let msg = match payload.downcast_ref::<&str>() {
            Some(s) => *s,
            None => match payload.downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };

        let msg = match info.location() {
            Some(location) => {
                format!("panicked at '{}', {}", msg, location)
            }
            None => format!("panicked at '{}'", msg),
        };

        let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x29c0, 0x0001))
            .strings(&[StringDescriptors::default()
                .manufacturer("Eny's Workshop")
                .product("Eny's Gameboy")
                .serial_number("EGB-0001")])
            .unwrap()
            .device_class(2) // from: https://www.usb.org/defined-class-codes
            .build();

        //serial.flush();

        let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
        let sio = hal::Sio::new(pac.SIO);

        let pins = hal::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let mut led_l = pins.gpio28.into_push_pull_output();
        let mut led_r = pins.gpio4.into_push_pull_output();

        usb_dev.poll(&mut [&mut serial]);
        serial.write(msg.as_bytes());
        serial.flush();

        led_l.set_high().unwrap();
        delay.delay_ms(330);
        led_l.set_low().unwrap();
        delay.delay_ms(1500);

        /*loop {
            led_l.set_high().unwrap();
            let out = serial.write(msg.as_bytes());
            led_r.set_state(out.is_err().into()).unwrap();
            led_l.set_low().unwrap();
            delay.delay_ms(250u32);
            led_r.set_low().unwrap();
            delay.delay_ms(250u32);
            //serial.write(msg.as_bytes());
        }*/

        let mut logged = false;

        loop {
            usb_dev.poll(&mut [&mut serial]);
            delay.delay_ms(5);
            if !logged && timer.get_counter().ticks() >= 4_000_000 {
                led_r.set_high().unwrap();
                serial.write(msg.as_bytes());
                serial.flush();
                logged = true;
            }
        }
    }
}

// End of file
