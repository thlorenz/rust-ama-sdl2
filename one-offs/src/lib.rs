use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use sdl2::Sdl;
use std::error::Error;
use std::path::Path;

pub fn init_renderer() -> Result<(Sdl, WindowCanvas), Box<dyn Error>> {
    let sdl_context: Sdl = sdl2::init()?;
    let video = sdl_context.video()?;

    let window = video
        .window("SDL One Offs", 1280, 960)
        // .position(-600, 0)
        .position_centered()
        .resizable()
        .build()?;

    let canvas = window.into_canvas().accelerated().present_vsync().build()?;

    Ok((sdl_context, canvas))
}

pub fn load_media<'a>(
    path: &'a str,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, Box<dyn Error>> {
    let surface = Surface::load_bmp(path)?;
    let texture = texture_creator.create_texture_from_surface(surface)?;
    Ok(texture)
}

pub enum Flip {
    None,
    Horizontal,
    Vertical,
}

pub struct Sprite<'a> {
    texture: Texture<'a>,
    width: u32,
    height: u32,
}

impl<'a> Sprite<'a> {
    pub fn from_surface(
        surface: Surface,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, Box<dyn Error>> {
        let width = surface.width();
        let height = surface.height();
        let texture = texture_creator.create_texture_from_surface(surface)?;

        Ok(Sprite {
            texture,
            width,
            height,
        })
    }

    pub fn load_from_file(
        image_path: &Path,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut surface = Surface::from_file(image_path)
            .unwrap_or_else(|_| panic!("FATAL: unable to load surface from file {:?}", image_path));

        surface.set_color_key(true, Color::RGB(0, 0xff, 0xff))?;
        Sprite::from_surface(surface, texture_creator)
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        self.texture.set_alpha_mod(alpha)
    }

    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.texture.set_blend_mode(blend_mode)
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.texture.set_color_mod(r, g, b);
    }

    pub fn render<R2>(
        &self,
        canvas: &mut WindowCanvas,
        x: i32,
        y: i32,
        clip: R2,
    ) -> Result<(), String>
    where
        R2: Into<Option<Rect>>,
    {
        self.render_ex(canvas, x, y, clip, 0.0, None, &Flip::None)
    }

    pub fn render_ex<R2, P>(
        &self,
        canvas: &mut WindowCanvas,
        x: i32,
        y: i32,
        clip: R2,
        angle: f64,
        center: P,
        flip: &Flip,
    ) -> Result<(), String>
    where
        R2: Into<Option<Rect>>,
        P: Into<Option<Point>>,
    {
        let (flip_horizontal, flip_vertical) = match flip {
            Flip::None => (false, false),
            Flip::Horizontal => (true, false),
            Flip::Vertical => (false, true),
        };
        match clip.into() {
            None => {
                let rect = Rect::new(x, y, self.width, self.height);
                canvas.copy_ex(
                    &self.texture,
                    None,
                    rect,
                    angle,
                    center,
                    flip_horizontal,
                    flip_vertical,
                )
            }
            Some(clip) => {
                let rect = Rect::new(x, y, clip.width(), clip.height());
                canvas.copy_ex(
                    &self.texture,
                    clip,
                    rect,
                    angle,
                    center,
                    flip_horizontal,
                    flip_vertical,
                )
            }
        }
    }
}
