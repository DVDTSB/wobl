use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Print, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::collections::HashSet;
use std::io::{Stdout, Write, stdout};
use std::time::{Duration, Instant};

use crate::backend::{Backend, Key};
use crate::cell::Cell;

pub struct CrosstermBackend {
    pressed_keys: HashSet<Key>,
    just_pressed: HashSet<Key>,
    released_keys: HashSet<Key>,
    milis: u64,
    stdout: Stdout,
}

impl CrosstermBackend {
    pub fn new(fps: u16) -> Self {
        let mut stdout = stdout();
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            crossterm::cursor::Hide
        )
        .unwrap();
        terminal::enable_raw_mode().unwrap();

        CrosstermBackend {
            pressed_keys: HashSet::new(),
            just_pressed: HashSet::new(),
            released_keys: HashSet::new(),
            milis: (1.0 / (fps as f32)) as u64,
            stdout,
        }
    }

    pub fn map_key_code(code: KeyCode) -> Key {
        match code {
            // Letters
            KeyCode::Char('a') => Key::A,
            KeyCode::Char('b') => Key::B,
            KeyCode::Char('c') => Key::C,
            KeyCode::Char('d') => Key::D,
            KeyCode::Char('e') => Key::E,
            KeyCode::Char('f') => Key::F,
            KeyCode::Char('g') => Key::G,
            KeyCode::Char('h') => Key::H,
            KeyCode::Char('i') => Key::I,
            KeyCode::Char('j') => Key::J,
            KeyCode::Char('k') => Key::K,
            KeyCode::Char('l') => Key::L,
            KeyCode::Char('m') => Key::M,
            KeyCode::Char('n') => Key::N,
            KeyCode::Char('o') => Key::O,
            KeyCode::Char('p') => Key::P,
            KeyCode::Char('q') => Key::Q,
            KeyCode::Char('r') => Key::R,
            KeyCode::Char('s') => Key::S,
            KeyCode::Char('t') => Key::T,
            KeyCode::Char('u') => Key::U,
            KeyCode::Char('v') => Key::V,
            KeyCode::Char('w') => Key::W,
            KeyCode::Char('x') => Key::X,
            KeyCode::Char('y') => Key::Y,
            KeyCode::Char('z') => Key::Z,

            // Numbers
            KeyCode::Char('0') => Key::Key0,
            KeyCode::Char('1') => Key::Key1,
            KeyCode::Char('2') => Key::Key2,
            KeyCode::Char('3') => Key::Key3,
            KeyCode::Char('4') => Key::Key4,
            KeyCode::Char('5') => Key::Key5,
            KeyCode::Char('6') => Key::Key6,
            KeyCode::Char('7') => Key::Key7,
            KeyCode::Char('8') => Key::Key8,
            KeyCode::Char('9') => Key::Key9,

            // Function keys
            KeyCode::F(1) => Key::F1,
            KeyCode::F(2) => Key::F2,
            KeyCode::F(3) => Key::F3,
            KeyCode::F(4) => Key::F4,
            KeyCode::F(5) => Key::F5,
            KeyCode::F(6) => Key::F6,
            KeyCode::F(7) => Key::F7,
            KeyCode::F(8) => Key::F8,
            KeyCode::F(9) => Key::F9,
            KeyCode::F(10) => Key::F10,
            KeyCode::F(11) => Key::F11,
            KeyCode::F(12) => Key::F12,

            // Arrows
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,

            // Control
            KeyCode::Esc => Key::Escape,
            KeyCode::Enter => Key::Enter,
            KeyCode::Tab => Key::Tab,
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Char(' ') => Key::Space,

            // Symbols
            KeyCode::Char('-') => Key::Minus,
            KeyCode::Char('=') => Key::Equals,
            KeyCode::Char('[') => Key::LeftBracket,
            KeyCode::Char(']') => Key::RightBracket,
            KeyCode::Char('\\') => Key::Backslash,
            KeyCode::Char(';') => Key::Semicolon,
            KeyCode::Char('\'') => Key::Apostrophe,
            KeyCode::Char('`') => Key::Grave,
            KeyCode::Char(',') => Key::Comma,
            KeyCode::Char('.') => Key::Period,
            KeyCode::Char('/') => Key::Slash,

            // Unknown / unmapped
            _ => Key::Unknown,
        }
    }

    fn record_key(&mut self, key_event: KeyEvent) {
        let mapped = Self::map_key_code(key_event.code);
        if mapped != Key::Unknown {
            if !self.pressed_keys.contains(&mapped) {
                self.just_pressed.insert(mapped);
            }
            self.pressed_keys.insert(mapped);
        }
    }

    fn release_key(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
        self.released_keys.insert(key);
    }
}

impl Backend for CrosstermBackend {
    fn wait_frame(&mut self) {
        //self.flush();

        self.just_pressed.clear();
        self.released_keys.clear();

        let frame_start = Instant::now();
        let frame_duration = Duration::from_millis(self.milis);

        while event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                self.record_key(key_event);
            }
        }

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    fn is_key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    fn is_key_just_pressed(&self, key: Key) -> bool {
        self.just_pressed.contains(&key)
    }

    fn is_key_just_released(&self, key: Key) -> bool {
        self.released_keys.contains(&key)
    }

    fn draw_cell(&mut self, x: u16, y: u16, cell: Cell) {
        let fg = crossterm::style::Color::Rgb {
            r: cell.fg.r,
            b: cell.fg.g,
            g: cell.fg.b,
        };

        let bg = crossterm::style::Color::Rgb {
            r: cell.bg.r,
            b: cell.bg.g,
            g: cell.bg.b,
        };
        execute!(
            self.stdout,
            cursor::MoveTo(x, y),
            SetForegroundColor(fg),
            SetBackgroundColor(bg),
            Print(cell.ch)
        )
        .unwrap();
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}

impl Drop for CrosstermBackend {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(
            self.stdout,
            terminal::LeaveAlternateScreen,
            crossterm::cursor::Show
        )
        .unwrap();
    }
}
