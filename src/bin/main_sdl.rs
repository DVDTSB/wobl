use sdl2::ttf::Sdl2TtfContext;
use wobl::{Attribute, Color, Key, Wobl, backend::SDLBackend};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context: &Sdl2TtfContext = Box::leak(Box::new(sdl2::ttf::init().unwrap()));

    // Create SDL backend with font size 30
    let backend = Box::new(SDLBackend::new(
        30,
        "resources/font.ttf",
        ttf_context,
        sdl_context,
    ));

    let mut wobl = Wobl::new(backend, "Colorful Wobl Demo", 50, 25, Some(30));
    let mut x = 10.0;
    let mut y = 10.0;

    loop {
        wobl.wait_frame();
        wobl.clear();

        if wobl.is_key_pressed(Key::Q) {
            break;
        }

        if wobl.is_key_pressed(Key::W) || wobl.is_key_pressed(Key::Up) {
            y -= 0.4;
        }
        if wobl.is_key_pressed(Key::S) || wobl.is_key_pressed(Key::Down) {
            y += 0.4;
        }

        if wobl.is_key_pressed(Key::A) || wobl.is_key_pressed(Key::Left) {
            x -= 0.8;
        }
        if wobl.is_key_pressed(Key::D) || wobl.is_key_pressed(Key::Right) {
            x += 0.8;
        }

        wobl.draw_text_atr(
            x as i32,
            y as i32,
            "hello",
            Color::Black,
            Color::White,
            &vec![Attribute::Italic],
        );

        wobl.draw_text_atr(
            x as i32 + 6,
            y as i32,
            "world",
            Color::Red,
            Color::White,
            &vec![Attribute::Bold],
        );
    }
}
