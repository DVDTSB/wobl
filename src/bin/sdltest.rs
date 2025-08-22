use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL2 Grid Test", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let cell_size = 32;
    let cols = 20;
    let rows = 15;

    let mut frame = 0;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        //canvas.clear();

        // Draw grid of squares with changing colors
        for y in 0..rows {
            for x in 0..cols {
                let bg_color = Color::RGB(
                    ((x * 12 + frame) % 256) as u8,
                    ((y * 16 + frame) % 256) as u8,
                    ((x * y + frame) % 256) as u8,
                );
                let fg_color = Color::RGB(
                    ((x * 8 + frame * 2) % 256) as u8,
                    ((y * 4 + frame * 3) % 256) as u8,
                    ((x * y * 2 + frame) % 256) as u8,
                );

                // Draw background
                canvas.set_draw_color(bg_color);
                canvas.fill_rect(Rect::new(
                    (x * cell_size) as i32,
                    (y * cell_size) as i32,
                    cell_size,
                    cell_size,
                ))?;

                // Draw foreground "cell" as a smaller square in the center
                canvas.set_draw_color(fg_color);
                let margin = 6;
                canvas.fill_rect(Rect::new(
                    (x * cell_size + margin) as i32,
                    (y * cell_size + margin) as i32,
                    cell_size - margin * 2,
                    cell_size - margin * 2,
                ))?;
            }
        }

        canvas.present();

        frame += 1;
        ::std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    Ok(())
}
