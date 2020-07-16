use lazy_foo::{init_renderer, load_media};
use sdl2::event::Event;

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    let texture_creator = canvas.texture_creator();
    let img_x = load_media("assets/x.bmp", &texture_creator).expect("FATAL: failed to load image.");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    // Start game loop
    'running: loop {
        // Check for an event on each loop iteration and break out of the loop if user
        // quit, i.e by closing the window.
        if let Some(Event::Quit { .. }) = event_pump.poll_event() {
            break 'running;
        }

        // Redraw the image on each loop iteration.
        // In a game things would update state/position, etc. and redraw differently each time.
        canvas.clear();
        canvas
            .copy(&img_x, None, None)
            .expect("FATAL: failed to draw to canvas.");
        canvas.present();
    }
}
