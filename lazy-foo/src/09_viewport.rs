use lazy_foo::{init_renderer, HEIGHT, WIDTH};
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::error::Error;

fn run(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/09_viewport/viewport.png")?;

    let top_left = Rect::new(0, 0, WIDTH / 2, HEIGHT / 2);
    let top_right = Rect::new((WIDTH / 2) as i32, 0, WIDTH / 2, HEIGHT / 2);
    let bottom = Rect::new(0, (HEIGHT / 2) as i32, WIDTH, HEIGHT / 2);

    'running: loop {
        if let Some(Event::Quit { .. }) = event_pump.poll_event() {
            break 'running;
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();

        canvas.set_viewport(top_left);
        canvas.copy(&texture, None, None)?;

        canvas.set_viewport(top_right);
        canvas.copy(&texture, None, None)?;

        canvas.set_viewport(bottom);
        canvas.copy(&texture, None, None)?;

        canvas.present();
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
