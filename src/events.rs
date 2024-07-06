use crate::games::GameConsole;

pub enum Event {
    BacklightBrightness(u16),
    LedL(u16),
    LedR(u16),
    LaunchGame(GameConsole),
}
