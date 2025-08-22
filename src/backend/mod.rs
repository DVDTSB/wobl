use crate::cell;

mod crossterm;

pub use crossterm::CrosstermBackend;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Numbers (top row)
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Control keys
    Escape,
    Enter,
    Tab,
    Backspace,
    Space,

    // Modifiers
    Shift,
    Ctrl,
    Alt,
    CapsLock,

    // Arrows
    Up,
    Down,
    Left,
    Right,

    // Other symbols
    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Grave, // `
    Comma,
    Period,
    Slash,

    // Anything not mapped
    Unknown,
}

pub trait Backend {
    fn is_key_pressed(&self, key: Key) -> bool;
    fn is_key_just_pressed(&self, key: Key) -> bool;
    fn is_key_just_released(&self, key: Key) -> bool;
    fn draw_cell(&mut self, x: u16, y: u16, cell: &cell::Cell);
    fn wait_frame(&mut self);
    fn flush(&mut self);
    fn set_fps(&mut self, fps: Option<u32>);
}
