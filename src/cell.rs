pub use crossterm::style::Attribute;
pub use crossterm::style::Color;

// defines one cell of the grid
#[derive(Clone, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub atr: Vec<Attribute>,
}

impl Cell {
    // makes a new cell
    pub fn new(ch: char, fg: Color, bg: Color, atr: Vec<Attribute>) -> Cell {
        Cell { ch, fg, bg, atr }
    }
    // makes an empty cell
    pub fn empty() -> Cell {
        Cell {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
            atr: vec![Attribute::Reset],
        }
    }
}
