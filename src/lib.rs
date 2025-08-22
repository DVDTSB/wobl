pub mod backend;
mod cell;

pub use backend::Key;
pub use cell::{Attribute, Cell, Color};

pub struct Wobl {
    width: u32,
    height: u32,
    buffer: Vec<Cell>,
    backend: Box<dyn backend::Backend>,
}

impl Wobl {
    /// Creates the engine object:) You can set the backend here!
    pub fn new(
        backend: Box<dyn backend::Backend>,
        name: &str,
        width: u32,
        height: u32,
        fps: Option<u32>,
    ) -> Self {
        let size = (width * height) as usize;
        let mut wobl = Self {
            width,
            height,
            buffer: vec![Cell::empty(); size],
            backend,
        };
        wobl.backend.set_fps(fps);
        wobl.backend.init(name, width, height);
        wobl
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn wait_frame(&mut self) {
        self.flush();
        self.backend.wait_frame();
    }

    pub fn set_fps(&mut self, fps: Option<u32>) {
        self.backend.set_fps(fps);
    }

    pub fn draw_cell(&mut self, x: i32, y: i32, cell: &Cell) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.buffer[(y * (self.width as i32) + x) as usize] = cell.clone();
        }
    }

    pub fn draw_text_atr(
        &mut self,
        x: i32,
        y: i32,
        text: &str,
        fg: Color,
        bg: Color,
        atr: &Vec<Attribute>,
    ) {
        let mut cx = x;
        let mut cy = y;
        for ch in text.chars() {
            if ch == '\n' {
                cy += 1;
                cx = x;
                continue;
            }
            self.draw_cell(cx, cy, &Cell::new(ch, fg, bg, atr.clone()));
            cx += 1;
        }
    }

    pub fn draw_text(&mut self, x: i32, y: i32, text: &str, fg: Color, bg: Color) {
        self.draw_text_atr(x, y, text, fg, bg, &Vec::new());
    }

    pub fn clear(&mut self) {
        self.buffer = vec![Cell::empty(); self.buffer.len()];
    }

    pub fn flush(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index(x, y);
                let back = self.buffer[idx].clone();
                self.backend.draw_cell(x, y, &back);
            }
        }
        self.backend.flush();
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.backend.is_key_pressed(key)
    }

    pub fn is_key_just_pressed(&self, key: Key) -> bool {
        self.backend.is_key_just_pressed(key)
    }

    pub fn is_key_just_released(&self, key: Key) -> bool {
        self.backend.is_key_just_released(key)
    }
}
