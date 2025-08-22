use wobl::{Attribute, Color, Key, Wobl, backend::CrosstermBackend};

fn main() {
    let backend = Box::new(CrosstermBackend::new());
    let mut wobl = Wobl::new(backend, "wobl", 40, 25, Some(60));

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
