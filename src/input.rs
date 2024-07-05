#[cfg(feature = "simulator")]
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

#[derive(Default)]
pub struct ButtonStatus {
    pub pressed: bool,
    pub just_released: bool,
    pub repeat: bool,
}

// TODO: shoulder buttons
#[derive(Default)]
pub struct InputStatus {
    pub up: ButtonStatus,
    pub down: ButtonStatus,
    pub left: ButtonStatus,
    pub right: ButtonStatus,
    pub a: ButtonStatus,
    pub b: ButtonStatus,
    pub start: ButtonStatus,
    pub select: ButtonStatus,
}
#[cfg(feature = "simulator")]
impl InputStatus {
    pub fn key_down(&mut self, key: Keycode, repeat: bool) {
        match key {
            Keycode::Up => {
                self.up = ButtonStatus {
                    pressed: true,
                    just_released: false,
                    repeat,
                }
            }
            Keycode::Down => {
                self.down = ButtonStatus {
                    pressed: true,
                    just_released: false,
                    repeat,
                }
            }
            Keycode::Left => {
                self.left = ButtonStatus {
                    pressed: true,
                    just_released: false,
                    repeat,
                }
            }
            Keycode::Right => {
                self.right = ButtonStatus {
                    pressed: true,
                    just_released: false,
                    repeat,
                }
            }
            Keycode::Z => {
                self.a = ButtonStatus {
                    pressed: true,
                    just_released: false,
                    repeat,
                }
            }
            Keycode::X => {
                self.b = ButtonStatus {
                    pressed: true,
                    just_released: false,
                    repeat,
                }
            }
            Keycode::Return => {
                self.start = ButtonStatus {
                    pressed: true,
                    just_released: false,
                    repeat,
                }
            }
            Keycode::Space => {
                self.select = ButtonStatus {
                    pressed: true,
                    just_released: false,
                    repeat,
                }
            }
            _ => {}
        }
    }

    pub fn key_up(&mut self, key: Keycode) {
        match key {
            Keycode::Up => {
                self.up = ButtonStatus {
                    pressed: false,
                    just_released: true,
                    repeat: false,
                }
            }
            Keycode::Down => {
                self.down = ButtonStatus {
                    pressed: false,
                    just_released: true,
                    repeat: false,
                }
            }
            Keycode::Left => {
                self.left = ButtonStatus {
                    pressed: false,
                    just_released: true,
                    repeat: false,
                }
            }
            Keycode::Right => {
                self.right = ButtonStatus {
                    pressed: false,
                    just_released: true,
                    repeat: false,
                }
            }
            Keycode::Z => {
                self.a = ButtonStatus {
                    pressed: false,
                    just_released: true,
                    repeat: false,
                }
            }
            Keycode::X => {
                self.b = ButtonStatus {
                    pressed: false,
                    just_released: true,
                    repeat: false,
                }
            }
            Keycode::Return => {
                self.start = ButtonStatus {
                    pressed: false,
                    just_released: true,
                    repeat: false,
                }
            }
            Keycode::Space => {
                self.select = ButtonStatus {
                    pressed: false,
                    just_released: true,
                    repeat: false,
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        self.up.just_released = false;
        self.down.just_released = false;
        self.left.just_released = false;
        self.right.just_released = false;
        self.a.just_released = false;
        self.b.just_released = false;
        self.start.just_released = false;
        self.select.just_released = false;
    }

    pub fn is_pressed(&self, button: Button) -> bool {
        match button {
            Button::Up => self.up.pressed,
            Button::Down => self.down.pressed,
            Button::Left => self.left.pressed,
            Button::Right => self.right.pressed,
            Button::A => self.a.pressed,
            Button::B => self.b.pressed,
            Button::Start => self.start.pressed,
            Button::Select => self.select.pressed,
        }
    }

    pub fn is_repeated(&self, button: Button) -> bool {
        match button {
            Button::Up => self.up.repeat,
            Button::Down => self.down.repeat,
            Button::Left => self.left.repeat,
            Button::Right => self.right.repeat,
            Button::A => self.a.repeat,
            Button::B => self.b.repeat,
            Button::Start => self.start.repeat,
            Button::Select => self.select.repeat,
        }
    }

    pub fn just_released(&self, button: Button) -> bool {
        match button {
            Button::Up => self.up.just_released,
            Button::Down => self.down.just_released,
            Button::Left => self.left.just_released,
            Button::Right => self.right.just_released,
            Button::A => self.a.just_released,
            Button::B => self.b.just_released,
            Button::Start => self.start.just_released,
            Button::Select => self.select.just_released,
        }
    }

    pub fn get_status(&self, button: Button) -> &ButtonStatus {
        match button {
            Button::Up => &self.up,
            Button::Down => &self.down,
            Button::Left => &self.left,
            Button::Right => &self.right,
            Button::A => &self.a,
            Button::B => &self.b,
            Button::Start => &self.start,
            Button::Select => &self.select,
        }
    }
}
