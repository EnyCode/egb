use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use games::Game;
use input::InputStatus;
use nes::cpu::{self, Mem, CPU};
use rand::Rng;
use tinytga::Tga;

mod games;
mod gui;
mod input;
mod nes;
mod util;

fn color(byte: u8) -> Rgb565 {
    match byte {
        0 => Rgb565::BLACK,
        1 => Rgb565::WHITE,
        2 | 9 => Rgb565::CSS_GRAY,
        3 | 10 => Rgb565::RED,
        4 | 11 => Rgb565::GREEN,
        5 | 12 => Rgb565::BLUE,
        6 | 13 => Rgb565::MAGENTA,
        7 | 14 => Rgb565::YELLOW,
        _ => Rgb565::CYAN,
    }
}

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x200..0x600 {
        let color_idx = cpu.mem_read(i as u16);
        let color = color(color_idx);
        let (b1, b2) = ((
            ((color.r() >> 3) & 0xF8) << 3 | ((color.g() >> 2) & 0xFC) >> 3,
            (((color.g() >> 2) & 0xFC) << 5) | ((color.b() >> 3) & 0xF8),
        ));
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            update = true;
        }
        frame_idx += 2;
    }

    update
}

fn main() -> Result<(), core::convert::Infallible> {
    let game_code = vec![
        0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02,
        0x85, 0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9,
        0x0f, 0x85, 0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85,
        0x00, 0xa5, 0xfe, 0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20,
        0x8d, 0x06, 0x20, 0xc3, 0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c,
        0x38, 0x06, 0xa5, 0xff, 0xc9, 0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0,
        0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60, 0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85,
        0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0, 0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01,
        0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02, 0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05,
        0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06, 0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00,
        0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07, 0xe6, 0x03, 0xe6, 0x03, 0x20,
        0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06, 0xb5, 0x11, 0xc5, 0x11,
        0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c, 0x35, 0x07, 0x60,
        0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02, 0x4a, 0xb0,
        0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9, 0x20,
        0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
        0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10,
        0xb0, 0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5,
        0x10, 0x29, 0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe,
        0x91, 0x00, 0x60, 0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10,
        0x60, 0xa2, 0x00, 0xea, 0xea, 0xca, 0xd0, 0xfb, 0x60,
    ];
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
    ));

    util::write_font();*/
    let display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 320));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Default)
        .pixel_spacing(0)
        .scale(3)
        .build();
    let window = Window::new("Snake", &output_settings);
    //let mut gui = gui::GUI::new(display, games);

    //gui.draw_background()?;
    //gui.create_window();

    let mut input = InputStatus::default();

    let mut cpu = CPU::new();
    cpu.load(game_code);
    cpu.reset();

    let mut screen_state = [0 as u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();

    cpu.run_with_callback(move |cpu| {
        cpu.mem_write(0xFE, rng.gen_range(1..16));

        if read_screen_state(cpu, &mut screen_state) {}
    });

    /*'running: loop {
        gui.update();
        input.update();

        for event in gui.events().unwrap() {
            match event {
                SimulatorEvent::KeyDown {
                    keycode,
                    keymod,
                    repeat,
                } => {
                    input.key_down(keycode, repeat);
                }
                SimulatorEvent::KeyUp {
                    keycode,
                    keymod,
                    repeat,
                } => {
                    input.key_up(keycode);
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        gui.update_input(&input).unwrap();
    }*/

    Ok(())
}
