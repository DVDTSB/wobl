pub mod backend;
mod cell;

pub use backend::Key;
pub use cell::{Attribute, Cell, Color};

pub struct Wobl {
    width: u16,
    height: u16,
    front_buffer: Vec<Cell>,
    back_buffer: Vec<Cell>,
    backend: Box<dyn backend::Backend>,
}

impl Wobl {
    /// Creates the engine object:) You can set the backend here!
    pub fn new(
        backend: Box<dyn backend::Backend>,
        width: u16,
        height: u16,
        fps: Option<u32>,
    ) -> Self {
        let size = (width * height) as usize;
        let mut wobl = Self {
            width,
            height,
            front_buffer: vec![Cell::empty(); size],
            back_buffer: vec![Cell::empty(); size],
            backend,
        };
        wobl.backend.set_fps(fps);
        wobl
    }

    fn index(&self, x: u16, y: u16) -> usize {
        (y * self.width + x) as usize
    }

    fn flush(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index(x, y);
                let back = self.back_buffer[idx].clone();
                let front = self.front_buffer[idx].clone();
                if back != front {
                    self.backend.draw_cell(x, y, &back);
                    self.front_buffer[idx] = back;
                }
            }
        }
        self.backend.flush();
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
            self.back_buffer[(y * (self.width as i32) + x) as usize] = cell.clone();
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
        self.back_buffer.iter_mut().for_each(|c| *c = Cell::empty());
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
