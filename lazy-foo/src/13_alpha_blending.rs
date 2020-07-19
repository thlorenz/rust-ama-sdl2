use std::error::Error;
use std::path::Path;

use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
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

    pub fn set_alpha(&mut self, alpha: u8) {
        self.texture.set_alpha_mod(alpha)
    }

    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.texture.set_blend_mode(blend_mode)
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
    let mut background = Sprite::new(
        "assets/13_alpha_blending/fadein.png".as_ref(),
        &texture_creator,
    )?;
    let mut foreground = Sprite::new(
        "assets/13_alpha_blending/fadeout.png".as_ref(),
        &texture_creator,
    )?;
    background.set_blend_mode(BlendMode::None);
    foreground.set_blend_mode(BlendMode::Blend);

    let mut a: i32 = 255;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => a = min(a + 32, 255),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => a = max(a - 32, 0),
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();

        background.render(canvas, 0, 0)?;
        foreground.set_alpha(a as u8);
        foreground.render(canvas, 0, 0)?;

        canvas.present();
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
