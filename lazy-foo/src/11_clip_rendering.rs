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
use sdl2::rect::Rect;

struct Sprite<'a> {
    texture: Texture<'a>,
}

impl<'a> Sprite<'a> {
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

        Ok(Sprite { texture })
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
    let sprite_sheet = Sprite::new(
        "assets/11_clip_rendering/dots.png".as_ref(),
        &texture_creator,
    )?;
    let clips = (
        Rect::new(0, 0, 100, 100),
        Rect::new(100, 0, 100, 100),
        Rect::new(0, 100, 100, 100),
        Rect::new(100, 100, 100, 100),
    );

    'running: loop {
        if let Some(Event::Quit { .. }) = event_pump.poll_event() {
            break 'running;
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();

        // Render top left sprite
        sprite_sheet.render(canvas, 0, 0, &clips.0)?;

        // Render top right sprite
        sprite_sheet.render(canvas, (WIDTH - clips.1.width()) as i32, 0, &clips.1)?;

        // Render bottom left sprite
        sprite_sheet.render(canvas, 0, (HEIGHT - clips.2.height()) as i32, &clips.2)?;

        // Render bottom right sprite
        sprite_sheet.render(
            canvas,
            (WIDTH - clips.3.width()) as i32,
            (HEIGHT - clips.3.height()) as i32,
            &clips.3,
        )?;

        canvas.present();
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
