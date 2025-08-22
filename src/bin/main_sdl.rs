use std::time::Duration;

use sdl2::ttf::Sdl2TtfContext;
use wobl::{Attribute, Color, Key, Wobl, backend::SDLBackend};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut ttf_context: &Sdl2TtfContext = Box::leak(Box::new(sdl2::ttf::init().unwrap()));

    // Create SDL backend
    let mut backend = Box::new(SDLBackend::new(
        16,
        "resources/font.ttf",
        ttf_context,
        sdl_context,
    ));

    let mut wobl = Wobl::new(backend, "wobl", 60, 40, Some(10));

    //wobl.clear();
    //

    loop {
        wobl.wait_frame();

        wobl.draw_text_atr(
            10,
            10,
            "hello wobl!",
            Color::Black,
            Color::White,
            &vec![Attribute::Italic, Attribute::Bold],
        );

        std::thread::sleep(Duration::from_millis(100));
    }
}
