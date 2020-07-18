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
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::cmp::{max, min};

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

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.texture.set_color_mod(r, g, b);
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
    let mut sprite = Sprite::new(
        "assets/12_color_modulation/colors.png".as_ref(),
        &texture_creator,
    )?;

    // Making them i32 so we don't panic when we over/under flow during
    // an operation before max/min applies.
    let mut r: i32 = 255;
    let mut g: i32 = 255;
    let mut b: i32 = 255;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => r = min(r + 32, 255),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => g = min(g + 32, 255),
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => b = min(b + 32, 255),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => r = max(r - 32, 0),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => g = max(g - 32, 0),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => b = max(b - 32, 0),
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();

        sprite.set_color(r as u8, g as u8, b as u8);
        sprite.render(canvas, 0, 0)?;

        canvas.present();
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
