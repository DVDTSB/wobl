use super::Backend;
use crate::{Cell, Color as CColor, Key};
use sdl2::{
    Sdl,
    event::Event,
    keyboard::Keycode,
    pixels::Color as SColor,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::{Font, FontStyle, Sdl2TtfContext},
    video::{Window, WindowContext},
};
use std::collections::HashSet;
use std::time::{Duration, Instant};

pub fn get_color(color: CColor) -> SColor {
    match color {
        CColor::Reset => SColor::RGB(0, 0, 0),
        CColor::Black => SColor::RGB(0, 0, 0),
        CColor::DarkGrey => SColor::RGB(128, 128, 128),
        CColor::Red => SColor::RGB(255, 0, 0),
        CColor::DarkRed => SColor::RGB(128, 0, 0),
        CColor::Green => SColor::RGB(0, 255, 0),
        CColor::DarkGreen => SColor::RGB(0, 128, 0),
        CColor::Yellow => SColor::RGB(255, 255, 0),
        CColor::DarkYellow => SColor::RGB(128, 128, 0),
        CColor::Blue => SColor::RGB(0, 0, 255),
        CColor::DarkBlue => SColor::RGB(0, 0, 128),
        CColor::Magenta => SColor::RGB(255, 0, 255),
        CColor::DarkMagenta => SColor::RGB(128, 0, 128),
        CColor::Cyan => SColor::RGB(0, 255, 255),
        CColor::DarkCyan => SColor::RGB(0, 128, 128),
        CColor::White => SColor::RGB(255, 255, 255),
        CColor::Grey => SColor::RGB(192, 192, 192),
        CColor::Rgb { r, g, b } => SColor::RGB(r, g, b),
        CColor::AnsiValue(v) => {
            let r = ((v >> 5) & 0b111) * 36;
            let g = ((v >> 2) & 0b111) * 36;
            let b = (v & 0b11) * 85;
            SColor::RGB(r, g, b)
        }
    }
}

pub struct SDLBackend<'ttf> {
    font_width: u32,
    font_height: u32,
    context: Sdl,
    canvas: Option<Canvas<Window>>,
    font: Font<'ttf, 'static>,
    texture_creator: Option<TextureCreator<WindowContext>>,

    pressed_keys: HashSet<Key>,
    just_pressed: HashSet<Key>,
    released_keys: HashSet<Key>,

    frame_start: Instant,
    frame_millis: u64,
}

impl<'ttf> SDLBackend<'ttf> {
    pub fn new(
        font_size: u32,
        font_path: &str,
        ttf_context: &'ttf Sdl2TtfContext,
        context: Sdl,
    ) -> Self {
        let font = ttf_context.load_font(font_path, font_size as u16).unwrap();

        let (font_width, font_height) = font.size_of("W").unwrap();

        SDLBackend {
            font_width,
            font_height,
            context,
            canvas: None,
            font,
            texture_creator: None,
            pressed_keys: HashSet::new(),
            just_pressed: HashSet::new(),
            released_keys: HashSet::new(),
            frame_start: Instant::now(),
            frame_millis: 16,
        }
    }

    fn map_keycode(code: Keycode) -> Key {
        match code {
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
            Keycode::Space => Key::Space,
            Keycode::Escape => Key::Escape,
            Keycode::Return => Key::Enter,
            Keycode::Backspace => Key::Backspace,
            Keycode::Tab => Key::Tab,
            Keycode::Up => Key::Up,
            Keycode::Down => Key::Down,
            Keycode::Left => Key::Left,
            Keycode::Right => Key::Right,
            _ => Key::Unknown,
        }
    }

    fn update_keys(&mut self) {
        self.just_pressed.clear();
        self.released_keys.clear();

        let mut event_pump = self.context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(k),
                    repeat: false,
                    ..
                } => {
                    let key = Self::map_keycode(k);
                    if !self.pressed_keys.contains(&key) {
                        self.just_pressed.insert(key);
                    }
                    self.pressed_keys.insert(key);
                }
                Event::KeyUp {
                    keycode: Some(k), ..
                } => {
                    let key = Self::map_keycode(k);
                    self.pressed_keys.remove(&key);
                    self.released_keys.insert(key);
                }
                _ => {}
            }
        }
    }
}

impl<'ttf> Backend for SDLBackend<'ttf> {
    fn init(&mut self, name: &str, width: u32, height: u32) {
        let video_subsystem = self.context.video().unwrap();
        let window = video_subsystem
            .window(name, width * self.font_width, height * self.font_height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        self.texture_creator = Some(canvas.texture_creator());
        self.canvas = Some(canvas);
    }

    fn wait_frame(&mut self) {
        if let Some(canvas) = &mut self.canvas {
            canvas.clear();
        }
        self.update_keys();
        let elapsed = self.frame_start.elapsed();
        let target = Duration::from_millis(self.frame_millis);
        if elapsed < target {
            std::thread::sleep(target - elapsed);
        }
        self.frame_start = Instant::now();
    }

    fn set_fps(&mut self, fps: Option<u32>) {
        self.frame_millis = fps.map(|f| (1000 / f) as u64).unwrap_or(16);
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
        if let Some(canvas) = &mut self.canvas {
            canvas.set_draw_color(get_color(cell.bg));
            canvas
                .fill_rect(Rect::new(
                    (x * self.font_width) as i32,
                    (y * self.font_height) as i32,
                    self.font_width,
                    self.font_height,
                ))
                .unwrap();

            if let Some(tc) = &self.texture_creator {
                //set font style
                let mut style = FontStyle::NORMAL;
                for &attr in &cell.atr {
                    match attr {
                        crate::Attribute::Bold => style |= FontStyle::BOLD,
                        crate::Attribute::Italic => style |= FontStyle::ITALIC,
                        crate::Attribute::Underlined => style |= FontStyle::UNDERLINE,
                        _ => (),
                    }
                }
                self.font.set_style(style);

                let surface = self
                    .font
                    .render(&cell.ch.to_string())
                    .blended(get_color(cell.fg))
                    .unwrap();

                let (glyph_width, glyph_height) = surface.size();
                let texture = tc.create_texture_from_surface(&surface).unwrap();

                let target = Rect::new(
                    (x * self.font_width) as i32,
                    (y * self.font_height) as i32,
                    glyph_width,
                    glyph_height,
                );

                canvas.copy(&texture, None, Some(target)).unwrap();
            }
        }
    }
    fn flush(&mut self) {
        if let Some(canvas) = &mut self.canvas {
            canvas.present();
        }
    }
}
