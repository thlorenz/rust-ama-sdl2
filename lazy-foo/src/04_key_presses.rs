use lazy_foo::{init_renderer, load_media};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::error::Error;

struct Textures<'a> {
    default: Texture<'a>,
    up: Texture<'a>,
    right: Texture<'a>,
    down: Texture<'a>,
    left: Texture<'a>,
}

fn load_all_media<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Textures<'a>, Box<dyn Error>> {
    let default = load_media("assets/04_key_presses/press.bmp", texture_creator)?;
    let up = load_media("assets/04_key_presses/up.bmp", texture_creator)?;
    let right = load_media("assets/04_key_presses/right.bmp", texture_creator)?;
    let down = load_media("assets/04_key_presses/down.bmp", texture_creator)?;
    let left = load_media("assets/04_key_presses/left.bmp", texture_creator)?;
    Ok(Textures {
        default,
        up,
        right,
        down,
        left,
    })
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    let texture_creator = canvas.texture_creator();
    let textures = load_all_media(&texture_creator).expect("FATAL: failed to load images.");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    let mut img = &textures.default;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => img = &textures.up,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => img = &textures.right,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => img = &textures.down,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => img = &textures.left,
                Event::KeyDown { .. } => img = &textures.default,
                _ => {}
            }
        }
        if let Some(Event::Quit { .. }) = event_pump.poll_event() {
            break 'running;
        }

        canvas.clear();
        canvas
            .copy(&img, None, None)
            .expect("FATAL: failed to draw to canvas.");
        canvas.present();
    }
}
