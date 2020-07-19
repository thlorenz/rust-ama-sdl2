use std::error::Error;
use std::path::Path;

use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use sdl2::Sdl;

use lazy_foo::{init_renderer, HEIGHT, WIDTH};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

struct SpriteSheet<'a> {
    texture: Texture<'a>,
}

impl<'a> SpriteSheet<'a> {
    pub fn new(
        image_path: &Path,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut surface = Surface::from_file(image_path).expect(&format!(
            "FATAL: unable to load surface from file {:?}",
            image_path
        ));

        surface.set_color_key(true, Color::RGB(0, 0xff, 0xff))?;

        let texture = texture_creator.create_texture_from_surface(surface)?;

        Ok(SpriteSheet { texture })
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        x: i32,
        y: i32,
        clip: &Rect,
    ) -> Result<(), String> {
        let rect = Rect::new(x, y, clip.width(), clip.height());
        canvas.copy(&self.texture, *clip, rect)
    }
}

fn run(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    let texture_creator = canvas.texture_creator();
    let sprite_sheet = SpriteSheet::new(
        "assets/14_animated_sprites/foo.png".as_ref(),
        &texture_creator,
    )?;

    let clips = [
        Rect::new(0, 0, 64, 205),
        Rect::new(64, 0, 64, 205),
        Rect::new(128, 0, 64, 205),
        Rect::new(196, 0, 64, 205),
    ];

    let mut frame = 0;
    'running: loop {
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

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();

        let current_clip = clips[frame / 4];
        let x = (WIDTH - current_clip.width()) / 2;
        let y = (HEIGHT - current_clip.height()) / 2;
        sprite_sheet.render(canvas, x as i32, y as i32, &current_clip)?;
        canvas.present();

        frame += 1;
        if frame / 4 >= clips.len() {
            frame = 0
        };
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
