use embedded_graphics::pixelcolor::Rgb565;
use tinytga::Tga;

pub enum GameConsole {
    GameBoy,
    GameBoyColor,
    GameBoyAdvanced,
    Sprig,
    Placeholder,
}

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
