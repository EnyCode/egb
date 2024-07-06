use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::Rgb565,
};
use tinytga::Tga;

#[derive(Debug, Clone, PartialEq)]
pub enum GameConsole {
    GameBoy,
    GameBoyColor,
    GameBoyAdvanced,
    NES,
    Sprig,
    // TODO: needed?
    Placeholder,
}

impl GameConsole {
    pub fn get_pos(&self, size: &Size, i: u8) -> Point {
        let s = match self {
            GameConsole::GameBoy => Size::new(82, 91),
            GameConsole::GameBoyColor => todo!(),
            GameConsole::GameBoyAdvanced => Size::new(106, 61),
            GameConsole::NES => Size::new(82, 91),
            GameConsole::Sprig => todo!(),
            GameConsole::Placeholder => panic!("Placeholder console should not be used"),
        };

        match i {
            0 => Point::new(
                (0 - s.width as i32) + 16,
                (size.height as i32 - s.height as i32) / 2,
            ),
            1 => Point::new(
                (size.width as i32 - s.width as i32) / 2,
                (size.height as i32 - s.height as i32) / 2,
            ),
            2 => Point::new(
                size.width as i32 - 16,
                (size.height as i32 - s.height as i32) / 2,
            ),
            3 => Point::new(
                2 * size.width as i32 - 16,
                (size.height as i32 - s.height as i32) / 2,
            ),
            255 => Point::new(
                -(size.width as i32) + 16,
                (size.height as i32 - s.height as i32) / 2,
            ),
            _ => panic!("Invalid index {:?} with console {:?}", i, self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub title: &'static str,
    pub console: GameConsole,
    pub image: Tga<'static, Rgb565>,
}

impl Game {
    pub fn new_gameboy(title: &'static str, image: Tga<'static, Rgb565>) -> Game {
        Game {
            title,
            console: GameConsole::GameBoy,
            image,
        }
    }

    pub fn new_gameboy_color(title: &'static str, image: Tga<'static, Rgb565>) -> Game {
        Game {
            title,
            console: GameConsole::GameBoyColor,
            image,
        }
    }

    pub fn new_gameboy_advanced(title: &'static str, image: Tga<'static, Rgb565>) -> Game {
        Game {
            title,
            console: GameConsole::GameBoyAdvanced,
            image,
        }
    }

    pub fn new_nes(title: &'static str, image: Tga<'static, Rgb565>) -> Game {
        Game {
            title,
            console: GameConsole::NES,
            image,
        }
    }

    pub fn new_sprig(title: &'static str, image: Tga<'static, Rgb565>) -> Game {
        Game {
            title,
            console: GameConsole::Sprig,
            image,
        }
    }

    pub fn new_placeholder() -> Game {
        Game {
            title: "Placeholder",
            console: GameConsole::Placeholder,
            image: Tga::from_slice(include_bytes!("assets/empty.tga")).unwrap(),
        }
    }

    pub fn get_console(&self) -> &GameConsole {
        &self.console
    }
    pub fn get_title(&self) -> &'static str {
        self.title
    }
    pub fn get_image(&self) -> Tga<'static, Rgb565> {
        self.image
    }
}
