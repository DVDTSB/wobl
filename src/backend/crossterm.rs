use crossterm::{
    cursor,
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    style::{Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal,
};

#[cfg(not(feature = "crossterm_events"))]
use device_query::{DeviceQuery, DeviceState};

#[cfg(feature = "crossterm_events")]
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use std::collections::HashSet;
use std::io::{Stdout, Write, stdout};
use std::time::{Duration, Instant};

use crate::cell::Cell;
use crate::{Key, backend::Backend};

pub struct CrosstermBackend {
    pressed_keys: HashSet<Key>,
    just_pressed: HashSet<Key>,
    released_keys: HashSet<Key>,
    #[cfg(not(feature = "crossterm_events"))]
    device_state: DeviceState,
    frame_start: Instant,
    milis: u64,
    stdout: Stdout,

    front_buffer: Vec<Cell>,
    back_buffer: Vec<Cell>,

    width: u32,
    height: u32,
}

impl CrosstermBackend {
    pub fn new() -> Self {
        let mut stdout = stdout();
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            crossterm::cursor::Hide,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES),
            //PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES),
        )
        .unwrap();
        terminal::enable_raw_mode().unwrap();

        CrosstermBackend {
            stdout,
            pressed_keys: HashSet::new(),
            just_pressed: HashSet::new(),
            released_keys: HashSet::new(),
            #[cfg(not(feature = "crossterm_events"))]
            device_state: DeviceState::new(),
            frame_start: Instant::now(),
            milis: 0,
            front_buffer: Vec::new(),
            back_buffer: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    #[cfg(not(feature = "crossterm_events"))]
    fn map_key(code: &device_query::Keycode) -> Key {
        use device_query::Keycode;
        match code {
            // Letters
            Keycode::A => Key::A,
            Keycode::B => Key::B,
            Keycode::C => Key::C,
            Keycode::D => Key::D,
            Keycode::E => Key::E,
            Keycode::F => Key::F,
            Keycode::G => Key::G,
            Keycode::H => Key::H,
            Keycode::I => Key::I,
            Keycode::J => Key::J,
            Keycode::K => Key::K,
            Keycode::L => Key::L,
            Keycode::M => Key::M,
            Keycode::N => Key::N,
            Keycode::O => Key::O,
            Keycode::P => Key::P,
            Keycode::Q => Key::Q,
            Keycode::R => Key::R,
            Keycode::S => Key::S,
            Keycode::T => Key::T,
            Keycode::U => Key::U,
            Keycode::V => Key::V,
            Keycode::W => Key::W,
            Keycode::X => Key::X,
            Keycode::Y => Key::Y,
            Keycode::Z => Key::Z,

            // Digits
            Keycode::Key0 => Key::Key0,
            Keycode::Key1 => Key::Key1,
            Keycode::Key2 => Key::Key2,
            Keycode::Key3 => Key::Key3,
            Keycode::Key4 => Key::Key4,
            Keycode::Key5 => Key::Key5,
            Keycode::Key6 => Key::Key6,
            Keycode::Key7 => Key::Key7,
            Keycode::Key8 => Key::Key8,
            Keycode::Key9 => Key::Key9,

            // Arrows
            Keycode::Up => Key::Up,
            Keycode::Down => Key::Down,
            Keycode::Left => Key::Left,
            Keycode::Right => Key::Right,

            // Function keys
            Keycode::F1 => Key::F1,
            Keycode::F2 => Key::F2,
            Keycode::F3 => Key::F3,
            Keycode::F4 => Key::F4,
            Keycode::F5 => Key::F5,
            Keycode::F6 => Key::F6,
            Keycode::F7 => Key::F7,
            Keycode::F8 => Key::F8,
            Keycode::F9 => Key::F9,
            Keycode::F10 => Key::F10,
            Keycode::F11 => Key::F11,
            Keycode::F12 => Key::F12,

            // Whitespace and control
            Keycode::Space => Key::Space,
            Keycode::Enter => Key::Enter,
            Keycode::Tab => Key::Tab,
            Keycode::Escape => Key::Escape,
            Keycode::Backspace => Key::Backspace,

            // Modifiers and symbols (if needed)
            Keycode::LShift | Keycode::RShift => Key::Shift,
            Keycode::LControl | Keycode::RControl => Key::Ctrl,
            Keycode::LAlt | Keycode::RAlt => Key::Alt,
            Keycode::CapsLock => Key::CapsLock,

            // Unknown / unhandled
            _ => Key::Unknown,
        }
    }
    #[cfg(not(feature = "crossterm_events"))]
    fn update_keys(&mut self) {
        let old_keys = self.pressed_keys.clone();

        let current_keys: HashSet<Key> = self
            .device_state
            .get_keys()
            .iter()
            .map(Self::map_key)
            .filter(|&x| x != Key::Unknown)
            .collect();

        self.just_pressed = current_keys.difference(&old_keys).cloned().collect();
        self.released_keys = old_keys.difference(&current_keys).cloned().collect();
        self.pressed_keys = current_keys;
    }

    #[cfg(feature = "crossterm_events")]
    fn map_key(code: KeyCode) -> Key {
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

    #[cfg(feature = "crossterm_events")]
    fn update_keys(&mut self) {
        self.just_pressed.clear();
        self.released_keys.clear();
        while event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                let key = Self::map_key(key_event.code);
                if key == Key::Unknown {
                    continue;
                }

                match key_event.kind {
                    KeyEventKind::Press => {
                        if !self.pressed_keys.contains(&key) {
                            self.just_pressed.insert(key);
                        }
                        self.pressed_keys.insert(key);
                    }
                    KeyEventKind::Release => {
                        self.pressed_keys.remove(&key);
                        self.released_keys.insert(key);
                    }
                    KeyEventKind::Repeat => {}
                }
            }
        }
    }

    fn draw_cell_internal(&mut self, x: u32, y: u32, cell: &Cell) {
        let fg = cell.fg;
        let bg = cell.bg;
        let atr = cell.atr.clone();
        for &atribute in atr.iter() {
            execute!(self.stdout, SetAttribute(atribute)).unwrap();
        }
        execute!(
            self.stdout,
            cursor::MoveTo(x as u16, y as u16),
            SetForegroundColor(fg),
            SetBackgroundColor(bg),
            Print(cell.ch)
        )
        .unwrap();
    }
}

impl Backend for CrosstermBackend {
    fn init(&mut self, _name: &str, width: u32, height: u32) {
        self.back_buffer = vec![Cell::empty(); (width * height) as usize];
        self.front_buffer = self.back_buffer.clone();
        self.width = width as u32;
        self.height = height as u32;
    }

    fn wait_frame(&mut self) {
        //self.flush();

        self.update_keys();

        let frame_duration = Duration::from_millis(self.milis);

        let elapsed = self.frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
        self.frame_start = Instant::now();
    }

    fn set_fps(&mut self, fps: Option<u32>) {
        if fps.is_none() {
            return;
        }
        self.milis = (1000.0 / (fps.unwrap() as f32)) as u64;
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

    fn draw_cell(&mut self, x: u32, y: u32, cell: &Cell) {
        self.front_buffer[(x + self.width * y) as usize] = cell.clone();
    }

    fn flush(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (x + self.width * y) as usize;

                let back = self.back_buffer[idx].clone();
                let front = self.front_buffer[idx].clone();

                if front != back {
                    self.draw_cell_internal(x, y, &front);
                }
            }
        }
        self.back_buffer = self.front_buffer.clone();
        self.front_buffer = vec![Cell::empty(); self.front_buffer.len()];
        self.stdout.flush().unwrap();
    }
}

impl Drop for CrosstermBackend {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(
            self.stdout,
            terminal::LeaveAlternateScreen,
            crossterm::cursor::Show,
            PopKeyboardEnhancementFlags
        )
        .unwrap();
    }
}
