use std::error::Error;

use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use sdl2::Sdl;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

// init method we created in ./02_image_on_screen.rs
pub fn init_renderer() -> Result<(Sdl, WindowCanvas), Box<dyn Error>> {
    let sdl_context: Sdl = sdl2::init()?;
    let video = sdl_context.video()?;

    let window = video
        .window("SDL Tutorial", WIDTH, HEIGHT)
        .position_centered()
        .build()?;

    let canvas = window.into_canvas().build()?;

    Ok((sdl_context, canvas))
}

// load_media method we created in ./02_image_on_screen.rs adapted to pass in path
pub fn load_media<'a>(
    path: &'a str,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, Box<dyn Error>> {
    let surface = Surface::load_bmp(path)?;
    let texture = texture_creator.create_texture_from_surface(surface)?;
    Ok(texture)
}
