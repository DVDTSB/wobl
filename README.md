# wobl

multi-backend textbased game engine :)

## use

```rust
use wobl::{Attribute, Color, Key, Wobl, backend::CrosstermBackend};

fn main() {
    let backend = Box::new(CrosstermBackend::new());
    let mut engine = Wobl::new(backend, 60, 30, Some(60));

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

- `crossterm` (woah terminal!): this should work pretty much everywhere: windows, linux (x11) and macos. if feature `crossterm_events` is enabled then it uses terminal events (`kitty` protocol), otherwise it uses `device_query` - enable it if using wayland (add `--features crossterm_events`)!
- `sdl`: for now - it just kinda works - i can definetly make more optimizations (like a texture atlas)

boilerplate can differ slightly from backend to backend, but only a couple of lines of code :)

- `crossterm`:
```rust
    let backend = Box::new(CrosstermBackend::new());
    let mut engine = Wobl::new(backend, 50, 25, Some(60));
```

-`sdl`:
```rust
    let sdl_context = sdl2::init().unwrap();
    let ttf_context: &Sdl2TtfContext = Box::leak(Box::new(sdl2::ttf::init().unwrap()));

    let backend = Box::new(SDLBackend::new(
        30,
        "resources/font.ttf",
        ttf_context,
        sdl_context,
    ));

    let mut wobl = Wobl::new(backend, "Colorful Wobl Demo", 50, 25, Some(60));
    loop {...}
```


# to do
- [] add more utilities:
    - [] line drawing
    - [] box drawing
    - [] autotile
- [] use font atlas thing in `sdl`
- [] maybe `winit` and `wgpu` backend

