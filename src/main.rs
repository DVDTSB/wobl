use wobl::{Color, Key, Wobl, backend::CrosstermBackend};

fn main() {
    let backend = Box::new(CrosstermBackend::new(60));

    let mut engine = Wobl::new(backend, 60, 40);

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

        engine.draw_text(
            x as u16,
            y as u16,
            "meow".into(),
            Color::from_vec(&vec![255, 255, 255]),
            Color::from_vec(&vec![255, 0, 0]),
        );

        frame += 1;
    }
}
