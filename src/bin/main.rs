use wobl::{Attribute, Color, Key, Wobl, backend::CrosstermBackend};

fn main() {
    println!("hi");
    let backend = Box::new(CrosstermBackend::new());
    let mut wobl = Wobl::new(backend, "wobl", 60, 40, Some(60));
    loop {
        wobl.wait_frame();

        //wobl.clear();

        if wobl.is_key_pressed(Key::Q) {
            break;
        }
        wobl.draw_text_atr(
            10,
            10,
            "hello wobl!",
            Color::Black,
            Color::White,
            &vec![Attribute::Italic, Attribute::Bold],
        );
    }
}
