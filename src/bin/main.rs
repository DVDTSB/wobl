use wobl::{Attribute, Color, Key, Wobl, backend};

fn main() {
    let backend = Box::new(backend::CrosstermBackend::new());
    let mut wobl = Wobl::new(backend, "wobl", 50, 25, Some(30));

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
