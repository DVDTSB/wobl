pub use crossterm::style::Attribute;
pub use crossterm::style::Color;

#[derive(Clone, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub atr: Vec<Attribute>,
}

impl Cell {
    pub fn new(ch: char, fg: Color, bg: Color, atr: Vec<Attribute>) -> Cell {
        Cell { ch, fg, bg, atr }
    }
    pub fn empty() -> Cell {
        Cell {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
            atr: vec![Attribute::Reset],
        }
    }
}
