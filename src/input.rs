use embedded_graphics_simulator::sdl2::Keycode;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

// TODO: shoulder buttons
#[derive(Default)]
pub struct InputStatus {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

impl InputStatus {
    pub fn key_down(&mut self, key: Keycode) {
        match key {
            Keycode::Up => self.up = true,
            Keycode::Down => self.down = true,
            Keycode::Left => self.left = true,
            Keycode::Right => self.right = true,
            Keycode::Z => self.a = true,
            Keycode::X => self.b = true,
            Keycode::Return => self.start = true,
            Keycode::Space => self.select = true,
            _ => {}
        }
    }

    pub fn key_up(&mut self, key: Keycode) {
        match key {
            Keycode::Up => self.up = false,
            Keycode::Down => self.down = false,
            Keycode::Left => self.left = false,
            Keycode::Right => self.right = false,
            Keycode::Z => self.a = false,
            Keycode::X => self.b = false,
            Keycode::Return => self.start = false,
            Keycode::Space => self.select = false,
            _ => {}
        }
    }

    pub fn is_pressed(&self, button: Button) -> bool {
        match button {
            Button::Up => self.up,
            Button::Down => self.down,
            Button::Left => self.left,
            Button::Right => self.right,
            Button::A => self.a,
            Button::B => self.b,
            Button::Start => self.start,
            Button::Select => self.select,
        }
    }
}
