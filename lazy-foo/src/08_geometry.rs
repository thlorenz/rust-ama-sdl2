use lazy_foo::{init_renderer, HEIGHT, WIDTH};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::error::Error;

fn run(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    'running: loop {
        if let Some(Event::Quit { .. }) = event_pump.poll_event() {
            break 'running;
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();

        // Render red filled quad
        let rect: Rect = Rect::new(
            (WIDTH / 4) as i32,
            (HEIGHT / 4) as i32,
            WIDTH / 2,
            HEIGHT / 2,
        );
        canvas.set_draw_color(Color::RGBA(0xff, 0x00, 0x00, 0xff));
        canvas.fill_rect(rect)?;

        // Render green outlined quad
        let rect: Rect = Rect::new(
            (WIDTH / 6) as i32,
            (HEIGHT / 6) as i32,
            WIDTH * 2 / 3,
            HEIGHT * 2 / 3,
        );
        canvas.set_draw_color(Color::RGBA(0x00, 0xff, 0x00, 0xff));
        canvas.draw_rect(rect)?;

        // Blue horizontal line
        canvas.set_draw_color(Color::RGBA(0x00, 0x00, 0xff, 0xff));
        let p1 = Point::new(0, (HEIGHT / 2) as i32);
        let p2 = Point::new(WIDTH as i32, (HEIGHT / 2) as i32);
        canvas.draw_line(p1, p2)?;

        // Draw vertical dotted yellow line
        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0x00, 0xff));
        for y in (0..HEIGHT).step_by(4) {
            canvas.draw_point(Point::new((WIDTH / 2) as i32, y as i32))?;
        }
        canvas.present();
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
