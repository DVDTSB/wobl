use crate::{Key, cell};

mod crossterm;
mod sdl;

pub use crossterm::CrosstermBackend;
pub use sdl::SDLBackend;

pub trait Backend {
    fn init(&mut self, name: &str, width: u32, height: u32);
    fn is_key_pressed(&self, key: Key) -> bool;
    fn is_key_just_pressed(&self, key: Key) -> bool;
    fn is_key_just_released(&self, key: Key) -> bool;
    fn draw_cell(&mut self, x: u32, y: u32, cell: &cell::Cell);
    fn wait_frame(&mut self);
    fn flush(&mut self);
    fn set_fps(&mut self, fps: Option<u32>);
}
