use lazy_foo::{init_renderer, load_media};
use sdl2::event::Event;

// As indicated in the Readme, not much new here, basically the same for us as tutorial
// 03_event_driven.rs
fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    let texture_creator = canvas.texture_creator();
    let img_x = load_media("assets/x.bmp", &texture_creator).expect("FATAL: failed to load image.");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    'running: loop {
        if let Some(Event::Quit { .. }) = event_pump.poll_event() {
            break 'running;
        }

        canvas.clear();
        canvas
            .copy(&img_x, None, None)
            .expect("FATAL: failed to draw to canvas.");
        canvas.present();
    }
}
