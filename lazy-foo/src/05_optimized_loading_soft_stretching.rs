use std::error::Error;

use sdl2::event::Event;
use sdl2::pixels::PixelFormat;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

use lazy_foo::{init_renderer, HEIGHT, WIDTH};
use std::convert::TryFrom;

pub fn load_surface<'a>(
    path: &'a str,
    window_pixel_format: PixelFormat,
) -> Result<Surface<'a>, Box<dyn Error>> {
    let loaded_surface = Surface::load_bmp(path)?;

    // Convert surface to screen format
    // - when you load a bitmap, it's typically loaded in a 24bit format
    // - modern displays are not 24bit by default
    // - blit an image that's 24bit onto a 32bit image, SDL will convert it every single time
    let optimized_surface = loaded_surface.convert(&window_pixel_format)?;
    Ok(optimized_surface)
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    // Getting window pixel format from its surface did not work here and failed with:
    // "No hardware accelerated renderers available"
    // let window_surface = canvas.window().surface(&event_pump).expect("..");
    // let window_pixel_format = window_surface.pixel_format();

    // Instead the proper way seems be to derive it from the pixel_format_enum
    // https://github.com/Rust-SDL2/rust-sdl2/pull/898
    let window_pixel_format_enum = canvas.window().window_pixel_format();
    // In my environment I could see in the debugger that window BitsPerPixel is indeed 32
    // while the surface loaded via `load_bmp` above had 24 BitsPerPixel
    let window_pixel_format = PixelFormat::try_from(window_pixel_format_enum)
        .expect("FATAL: unable to get window pixel format");

    let optimized_surface = load_surface(
        "assets/05_optimized_loading_soft_stretching/stretch.bmp",
        window_pixel_format,
    )
    .expect("FATAL: failed to load image surface.");

    // Create a stretched version of the surface
    // (NOTE: here we are stretching to half the window width to verify that
    // it actually is stretching, as by default the image already fills the window)
    let stretch_rect = Rect::new(0, 0, WIDTH / 2, HEIGHT);
    let mut stretched_surface = Surface::new(WIDTH, HEIGHT, window_pixel_format_enum)
        .expect("FATAL: failed to created stretch surface");
    optimized_surface
        .blit_scaled(None, &mut stretched_surface, Some(stretch_rect))
        .expect("FATAL: failed to blit surface");

    // We could operate on the window surface instead via `WindowSurfaceRef::update_window`
    // which would be closer to `SDL_UpdateWindowSurface` used in the tutorial, however
    // we like to stick to `canvas` operations
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&stretched_surface)
        .expect("FATAL: failed to create texture.");

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
