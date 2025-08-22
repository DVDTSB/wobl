use wobl::{Attribute, Color, Key, Wobl, backend::CrosstermBackend};

fn main() {
    let backend = Box::new(CrosstermBackend::new());

    let mut engine = Wobl::new(backend, 60, 40, Some(60));

    let mut frame = 0;

    let mut x = 10.0;
    let mut y = 10.0;

    loop {
        engine.wait_frame();

        engine.clear();

        if engine.is_key_pressed(Key::Q) {
            break;
        }
        if engine.is_key_pressed(Key::W) {
            y -= 0.01;
        }
        if engine.is_key_pressed(Key::S) {
            y += 0.01;
        }

        if engine.is_key_pressed(Key::A) {
            x -= 0.01;
        }
        if engine.is_key_pressed(Key::D) {
            x += 0.01;
        }

        engine.draw_text_atr(
            x as i32,
            y as i32,
            &format!("{frame}"),
            Color::from((255, 255, 255)),
            Color::from(((255.0 * (frame as f32 / 120.0).sin()).abs() as u8, 0, 0)),
            &vec![Attribute::Italic],
        );

        frame += 1;
    }
}
