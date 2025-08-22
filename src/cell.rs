#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_vec(v: &Vec<u8>) -> Color {
        let mut v = v.clone();
        while v.len() < 3 {
            v.push(0);
        }

        Color {
            r: v[0],
            g: v[1],
            b: v[2],
        }
    }
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    pub fn new(ch: char, fg: Color, bg: Color) -> Cell {
        Cell { ch, fg, bg }
    }
    pub fn empty() -> Cell {
        Cell {
            ch: ' ',
            fg: Color::new(0, 0, 0),
            bg: Color::new(0, 0, 0),
        }
    }
}
