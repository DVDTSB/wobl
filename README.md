# wobl

multi-backend textbased game engine :)

## use

```rust
use wobl::{Attribute, Color, Key, Wobl, backend::CrosstermBackend};

fn main() {
    let backend = Box::new(CrosstermBackend::new());
    let mut engine = Wobl::new(backend, 60, 40, Some(60));

    loop {
        engine.wait_frame();

        engine.clear();

        if engine.is_key_pressed(Key::Q) {
            break;
        }
        engine.draw_text_atr(
            10,
            10,
            "hello wobl!",
            Color::Black,
            Color::White,
            &vec![Attribute::Italic, Attribute::Bold],
        );
    }
}
```

## backends

backends are quite straight forward to implement. here are the included ones!

- `crossterm` (woah terminal!): this should work pretty much everywhere: windows, linux (x11) and macos. if feature `crossterm_events` is enabled then it uses terminal events (kitty protocol), otherwise it uses `device_query` - enable it if using wayland!
- `sdl`: for now - it just kinda works - i can definetly make more optimizations (like a texture atlas)





