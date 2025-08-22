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

    let colors = [
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Yellow,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::Grey,
        Color::DarkRed,
        Color::DarkGreen,
        Color::DarkBlue,
        Color::DarkYellow,
    ];

    loop {
        wobl.wait_frame();
        wobl.clear();

        if wobl.is_key_pressed(Key::Q) {
            break;
        }

        // Draw colorful text
        for (i, &fg) in colors.iter().enumerate() {
            let bg = if i % 2 == 0 {
                Color::Black
            } else {
                Color::White
            };
            let attr = if i % 3 == 0 {
                vec![Attribute::Bold]
            } else if i % 3 == 1 {
                vec![Attribute::Italic]
            } else {
                Vec::new()
            };
            wobl.draw_text_atr(
                0,
                0 + i as i32 * 2,
                &format!("Colorful line {}", i + 1),
                fg,
                bg,
                &attr,
            );
        }
    }
}
