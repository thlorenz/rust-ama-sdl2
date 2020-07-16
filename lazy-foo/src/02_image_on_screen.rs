use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use sdl2::Sdl;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn init() -> Result<(Sdl, WindowCanvas), Box<dyn Error>> {
    // Initialize SDL
    let sdl_context: Sdl = sdl2::init()?;
    let video = sdl_context.video()?;

    // Create Window
    let window = video
        .window("SDL Tutorial", WIDTH, HEIGHT)
        .position_centered()
        // .allow_highdpi() creates the window in high-DPI mode if available
        .build()?;

    // Create Canvas to render
    let canvas = window
        .into_canvas()
        // .accelerated()   if we want to leverage the GPU to render
        // .software()      to enforce Software rendering (default and opposite of accelerated)
        // .present_vsync() if we want to synchronize renderer `present` with display refresh rate
        //                  it only has an effect in accelerated mode.
        .build()?;

    // window is still accessible via `canvas.window()` since it 'owns' it, so returning
    // sdl_context and canvas is sufficient
    Ok((sdl_context, canvas))
}

fn load_media(texture_creator: &TextureCreator<WindowContext>) -> Result<Texture, Box<dyn Error>> {
    // Load Image
    let surface = Surface::load_bmp("assets/hello_rust_sdl.bmp")?;
    // Convert to Texture so we can render it
    // Same as: let texture = surface.as_texture(texture_creator)?;
    let texture = texture_creator.create_texture_from_surface(surface)?;
    Ok(texture)
}

fn main() {
    let (sdl_context, mut canvas) = init().expect("FATAL: failed to initialize window and canvas.");
    let texture_creator = canvas.texture_creator();
    let img_hello_world = load_media(&texture_creator).expect("FATAL: failed to load image.");

    canvas.clear();

    // Providing None, None for src and dst as we want to fill the window with the
    // entire image.
    // If `src` is `None`, the entire texture is copied.
    // If `dst` is `None`, the texture will be stretched to fill the given rectangle.
    canvas
        .copy(&img_hello_world, None, None)
        .expect("FATAL: failed to draw to canvas.");

    canvas.present();

    // If we don't init the event pump and pump events at least once, nothing will
    // happen and no window even show.
    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    event_pump.pump_events();
    sleep(Duration::new(2, 0));

    // We do not need to implement a `close` method like the original tutorial
    // does since Rust cleans things up for us.
}
