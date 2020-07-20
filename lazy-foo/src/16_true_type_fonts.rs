use lazy_foo::{init_renderer, Sprite, HEIGHT, WIDTH};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::error::Error;

fn run(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    let texture_creator = canvas.texture_creator();
    let ttf = sdl2::ttf::init()?;
    let font = ttf.load_font("assets/16_true_type_fonts/lazy.ttf", 24)?;
    let text = Sprite::load_from_rendered_text(
        "The quick brown fox jumps over the lazy dog",
        Color::BLACK,
        &font,
        &texture_creator,
    )?;

    let center_x = ((WIDTH - text.width()) / 2) as i32;
    let center_y = ((HEIGHT - text.height()) / 2) as i32;

    'running: loop {
        if let Some(Event::Quit { .. }) = event_pump.poll_event() {
            break 'running;
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();
        text.render(canvas, center_x, center_y, None)?;
        canvas.present();
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
