pub mod backend;
mod cell;

use std::collections::HashSet;

pub use backend::Key;
pub use cell::{Cell, Color};

pub struct Wobl {
    width: u16,
    height: u16,
    front_buffer: Vec<Cell>,
    back_buffer: Vec<Cell>,
    backend: Box<dyn backend::Backend>,
}

impl Wobl {
    pub fn new(backend: Box<dyn backend::Backend>, width: u16, height: u16) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            front_buffer: vec![Cell::empty(); size],
            back_buffer: vec![Cell::empty(); size],
            backend,
        }
    }

    fn index(&self, x: u16, y: u16) -> usize {
        (y * self.width + x) as usize
    }

    pub fn draw_cell(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.width && y < self.height {
            self.back_buffer[(y * self.width + x) as usize] = cell;
        }
    }

    pub fn draw_text(&mut self, x: u16, y: u16, text: &str, fg: Color, bg: Color) {
        let mut cx = x;
        let mut cy = y;
        for ch in text.chars() {
            if ch == '\n' {
                cy += 1;
                cx = x;
                continue;
            }
            self.draw_cell(cx, cy, Cell::new(ch, fg, bg));
            cx += 1;
        }
    }

    pub fn clear(&mut self) {
        self.back_buffer.iter_mut().for_each(|c| *c = Cell::empty());
    }

    pub fn flush(&mut self) {
        // Compare front & back buffers and only send differences
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index(x, y);
                let back = self.back_buffer[idx];
                let front = self.front_buffer[idx];
                if back != front {
                    self.backend.draw_cell(x, y, back);
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
