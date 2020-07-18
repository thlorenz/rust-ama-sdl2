use lazy_foo::init_renderer;
use sdl2::event::Event;
use sdl2::image;
use sdl2::image::{LoadSurface, LoadTexture};
use sdl2::pixels::PixelFormat;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use std::convert::TryFrom;
use std::error::Error;

// The way we've been loading images and converting to textures all along.
// A bit more verbose than the `direct_texture_load` way, but allows converting
// surface to PixelFormat
fn load_image<'a>(
    path: &str,
    format: PixelFormat,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, Box<dyn Error>> {
    let image_surface: Surface = LoadSurface::from_file(path)?;
    let image_surface = image_surface.convert(&format)?;

    let texture = image_surface.as_texture(texture_creator)?;
    Ok(texture)
}

// Shortcut to load a texture used in rust-sdl2 image demo example.
// Does not allow setting the PixelFormat.
#[allow(dead_code)]
fn direct_texture_load<'a>(
    path: &str,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    texture_creator.load_texture(path)
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");

    let init_flags = image::InitFlag::PNG;
    let _image_ctx = image::init(init_flags).expect("FATAL: failed to initialize sdl2:image");

    let texture_creator = canvas.texture_creator();

    // Alternative to the below 7 lines is to use the shortcut to load a texture:
    // let texture = direct_texture_load("assets/06_loading_pngs/loaded.png", &texture_creator)
    let window_pixel_format = PixelFormat::try_from(canvas.window().window_pixel_format())
        .expect("FATAL: failed to obtain window pixel format");
    let texture = load_image(
        "assets/06_loading_pngs/fixed.png",
        window_pixel_format,
        &texture_creator,
    )
    .expect("FATAL: failed to load png texture");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");
    'running: loop {
        if let Some(Event::Quit { .. }) = event_pump.poll_event() {
            break 'running;
        }

        canvas.clear();
        canvas
            .copy(&texture, None, None)
            .expect("FATAL: failed to draw to canvas.");
        canvas.present();
    }
}
