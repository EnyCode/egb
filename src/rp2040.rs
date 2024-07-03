use cortex_m::delay::Delay;
use embedded_graphics::{
    geometry::Point,
    image::{Image, ImageRaw, ImageRawLE},
    pixelcolor::{IntoStorage, Rgb565, RgbColor},
    Drawable,
};
use rp2040_hal::{
    self as hal,
    fugit::RateExtU32,
    gpio::{
        bank0::{Gpio16, Gpio18, Gpio19},
        FunctionSio, FunctionSpi, Pin, PinId, PullDown, SioOutput,
    },
    spi::Enabled,
};

use defmt::*;
use defmt_rtt as _;
use embedded_graphics::draw_target::DrawTarget;
use embedded_hal::{digital::OutputPin, spi::SpiDevice};
use hal::entry;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
// use sparkfun_pro_micro_rp2040 as bsp;

use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use st7735_lcd::{Orientation, ST7735};

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

pub struct Hardware<PIN, DIS, Led_L = PIN, Led_R, Backlight = PIN>
where
    PIN: OutputPin + PinId,
    DIS: DrawTarget,
{
    peripherals: pac::Peripherals,
    core: pac::CorePeripherals,
    clocks: hal::clocks::ClocksManager,
    pins: hal::gpio::Pins,
    led_pins: LedPins<Led_L, Led_R, Backlight>,
    delay: Delay,
    // TODO: move to type parameter
    display: DIS,
}

pub struct ButtonPins {}
pub struct LedPins<Led_L, Led_R = Led_L, Backlight = Led_L>
where
    Led_L: OutputPin + PinId,
{
    pub led_l: Led_L,
    pub led_r: Led_R,
    pub lcd_backlight: Backlight,
}

impl<PIN, DIS> Hardware<PIN, DIS>
where
    PIN: OutputPin + PinId,
    DIS: DrawTarget,
{
    pub fn init() -> Self {
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();

        // Set up the watchdog driver - needed by the clock setup code
        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

        // Configure the clocks
        let clocks = hal::clocks::init_clocks_and_plls(
            XTAL_FREQ_HZ,
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

        let spi_sclk = pins.gpio18.into_function::<hal::gpio::FunctionSpi>();
        let spi_mosi = pins.gpio19.into_function::<hal::gpio::FunctionSpi>();
        let spi_miso = pins.gpio16.into_function::<hal::gpio::FunctionSpi>();
        let spi = hal::Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));

        let mut lcd_led = pins.gpio17.into_push_pull_output();
        let mut _led = pins.gpio25.into_push_pull_output();

        let mut l_led = pins.gpio28.into_push_pull_output();
        let mut r_led = pins.gpio4.into_push_pull_output();

        let dc = pins.gpio22.into_push_pull_output();
        let rst = pins.gpio26.into_push_pull_output();

        // Exchange the uninitialised SPI driver for an initialised one
        let spi = spi.init(
            &mut pac.RESETS,
            clocks.peripheral_clock.freq(),
            16.MHz(),
            &embedded_hal::spi::MODE_0,
        );

        let mut disp: ST7735<
            rp2040_hal::Spi<
                rp2040_hal::spi::Enabled,
                pac::SPI0,
                (
                    Pin<Gpio19, FunctionSpi, PullDown>,
                    Pin<Gpio16, FunctionSpi, PullDown>,
                    Pin<Gpio18, FunctionSpi, PullDown>,
                ),
            >,
            Pin<
                rp2040_hal::gpio::bank0::Gpio22,
                rp2040_hal::gpio::FunctionSio<rp2040_hal::gpio::SioOutput>,
                PullDown,
            >,
            Pin<
                rp2040_hal::gpio::bank0::Gpio26,
                rp2040_hal::gpio::FunctionSio<rp2040_hal::gpio::SioOutput>,
                PullDown,
            >,
        > = ST7735::new(spi, dc, rst, true, false, 160, 128);
        let mut disp_cs = pins
            .gpio20
            .into_push_pull_output_in_state(hal::gpio::PinState::Low);
        disp_cs.set_low().unwrap();

        disp.init(&mut delay).unwrap();
        disp.set_orientation(&Orientation::Landscape).unwrap();
        disp.clear(Rgb565::BLACK).unwrap();
        //disp_cs.set_high().unwrap();
        disp.set_offset(0, 25);

        let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("ferris.raw"), 86);

        let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));

        image.draw(&mut disp).unwrap();

        //disp_cs.set_high().unwrap();

        //disp.set_pixel(64, 64, Rgb565::RED.into_storage()).unwrap();

        // Wait until the background and image have been rendered otherwise
        // the screen will show random pixels for a brief moment
        lcd_led.set_high().unwrap();
        l_led.set_high().unwrap();

        Self {
            peripherals: pac,
            core,
            clocks,
            pins,
            led_pins: LedPins {
                led_l: l_led,
                led_r: r_led,
                lcd_backlight: lcd_led,
            },
            delay,
            display: disp,
        }
    }
}

/*#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().raw());

    // The single-cycle I/O block controls our GPIO pins
    let sio = Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // These are implicitly used by the spi driver if they are in the correct mode
    let spi_sclk = pins.gpio18.into_function::<hal::gpio::FunctionSpi>();
    let spi_mosi = pins.gpio19.into_function::<hal::gpio::FunctionSpi>();
    let spi_miso = pins.gpio16.into_function::<hal::gpio::FunctionSpi>();
    let spi = hal::Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));

    let mut lcd_led = pins.gpio17.into_push_pull_output();
    let mut led = pins.gpio25.into_push_pull_output();
    let dc = pins.gpio22.into_push_pull_output();
    let rst = pins.gpio26.into_push_pull_output();

    // Exchange the uninitialised SPI driver for an initialised one
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16.MHz(),
        &embedded_hal::spi::MODE_1,
    );

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);

    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();
    disp.set_offset(0, 25);

    let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("ferris.raw"), 86);

    let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));

    image.draw(&mut disp).unwrap();

    // disp.set_pixel(64, 64, Rgb565::RED.into_storage()).unwrap();

    // Wait until the background and image have been rendered otherwise
    // the screen will show random pixels for a brief moment
    lcd_led.set_high().unwrap();
    led.set_high().unwrap();

    //let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);
    //disp.hard_reset(&mut delay).unwrap();
    //disp.init(&mut delay).unwrap();
    //disp.set_orientation(&Orientation::Landscape).unwrap();
    //disp.hard_reset(&mut delay).unwrap();
    //disp.clear(Rgb565::GREEN).unwrap();
    //disp.set_offset(0, 25);
    //let color = Rgb565::RED;
    //let (b1, b2) = (
    //    (color.r() << 3) | (color.g() >> 2),
    //    (color.g() & 0b11) << 5 | color.b(),
    //);
    //disp.set_pixel(0, 0, ((b1 as u16) << 8) | b2 as u16)
    //    .unwrap();

    //let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("ferris.raw"), 86);

    //let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));

    //image.draw(&mut disp).unwrap();

    delay.delay_ms(100);

    // Wait until the background and image have been rendered otherwise
    // the screen will show random pixels for a brief moment

    loop {
        continue;
    }
}*/

// End of file
