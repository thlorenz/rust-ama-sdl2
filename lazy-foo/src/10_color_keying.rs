use std::error::Error;
use std::path::Path;

use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use sdl2::Sdl;

use lazy_foo::init_renderer;
use sdl2::rect::Rect;

struct Sprite<'a> {
    texture: Texture<'a>,
    width: u32,
    height: u32,
}

impl<'a> Sprite<'a> {
    pub fn new(
        image_path: &Path,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, Box<dyn Error>> {
        // Alternative: let surface = image::LoadSurface::from_file(image_path);
        let mut surface = Surface::from_file(image_path).expect(&format!(
            "FATAL: unable to load surface from file {:?}",
            image_path
        ));

        surface.set_color_key(true, Color::RGB(0, 0xff, 0xff))?;

        let width = surface.width();
        let height = surface.height();
        let texture = texture_creator.create_texture_from_surface(surface)?;

        Ok(Sprite {
            texture,
            width,
            height,
        })
    }

    pub fn render(&self, canvas: &mut WindowCanvas, x: i32, y: i32) -> Result<(), String> {
        let rect = Rect::new(x, y, self.width, self.height);
        canvas.copy(&self.texture, None, rect)
    }
}

fn run(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    let texture_creator = canvas.texture_creator();
    let bg_sprite = Sprite::new(
        "assets/10_color_keying/background.png".as_ref(),
        &texture_creator,
    )?;
    let foo_sprite = Sprite::new("assets/10_color_keying/foo.png".as_ref(), &texture_creator)?;

    'running: loop {
        if let Some(Event::Quit { .. }) = event_pump.poll_event() {
            break 'running;
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();

        bg_sprite.render(canvas, 0, 0)?;
        foo_sprite.render(canvas, 240, 190)?;

        canvas.present();
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
