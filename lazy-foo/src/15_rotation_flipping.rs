use std::error::Error;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use lazy_foo::{init_renderer, Flip, Sprite, HEIGHT, WIDTH};
use sdl2::keyboard::Keycode;

fn run(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    let texture_creator = canvas.texture_creator();
    let sprite = Sprite::load_from_file(
        "assets/15_rotation_flipping/arrow.png".as_ref(),
        &texture_creator,
    )?;

    let mut degrees: f64 = 0.0;
    let mut flip = Flip::None;
    let center_x = ((WIDTH - sprite.width()) / 2) as i32;
    let center_y = ((HEIGHT - sprite.height()) / 2) as i32;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::A) => degrees -= 30.0,
                    Some(Keycode::D) => degrees += 30.0,
                    Some(Keycode::Q) => flip = Flip::Horizontal,
                    Some(Keycode::W) => flip = Flip::None,
                    Some(Keycode::E) => flip = Flip::Vertical,
                    _ => {}
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();

        // Render arrow possibly rotated and/or flipped
        sprite.render_ex(canvas, center_x, center_y, None, degrees, None, &flip)?;

        canvas.present();
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
