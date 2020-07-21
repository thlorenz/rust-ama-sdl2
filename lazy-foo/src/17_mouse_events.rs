use lazy_foo::{init_renderer, Sprite, HEIGHT, WIDTH};
use sdl2::event::Event;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::collections::HashSet;
use std::error::Error;

const BUTTON_WIDTH: u32 = 300;
const BUTTON_HEIGHT: u32 = 200;

enum ButtonType {
    MouseOut,
    MouseOver,
    MouseDown,
    MouseUp,
}

struct Button<'a> {
    rect: Rect,
    button_type: ButtonType,
    sprite: &'a Sprite<'a>,
}

impl<'a> Button<'a> {
    pub fn new(x: i32, y: i32, width: u32, height: u32, sprite: &'a Sprite) -> Button<'a> {
        let rect = Rect::new(x, y, width, height);

        Button {
            rect,
            button_type: ButtonType::MouseOut,
            sprite,
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let clip = match self.button_type {
            ButtonType::MouseOut => Rect::new(0, 0, BUTTON_WIDTH, BUTTON_HEIGHT),
            ButtonType::MouseOver => {
                Rect::new(0, BUTTON_HEIGHT as i32, BUTTON_WIDTH, BUTTON_HEIGHT)
            }
            ButtonType::MouseDown => {
                Rect::new(0, BUTTON_HEIGHT as i32 * 2, BUTTON_WIDTH, BUTTON_HEIGHT)
            }
            ButtonType::MouseUp => {
                Rect::new(0, BUTTON_HEIGHT as i32 * 3, BUTTON_WIDTH, BUTTON_HEIGHT)
            }
        };
        self.sprite.render(canvas, self.rect.x, self.rect.y, clip)
    }

    pub fn on_mouse_event(&mut self, event: &Event, mouse_state: &MouseState) {
        if self.is_mouse_inside(mouse_state.x(), mouse_state.y()) {
            match event {
                Event::MouseMotion { .. } => self.button_type = ButtonType::MouseOver,
                Event::MouseButtonDown { .. } => self.button_type = ButtonType::MouseDown,
                Event::MouseButtonUp { .. } => self.button_type = ButtonType::MouseUp,
                _ => {}
            }
        } else {
            self.button_type = ButtonType::MouseOut;
        }
    }

    fn is_mouse_inside(&self, x: i32, y: i32) -> bool {
        self.rect.contains_point(Point::new(x, y))
    }
}

fn log_mouse_button_state(
    mouse_state: &MouseState,
    prev_buttons: HashSet<MouseButton>,
) -> HashSet<MouseButton> {
    let buttons = mouse_state.pressed_mouse_buttons().collect();
    let new_buttons = &buttons - &prev_buttons;
    let old_buttons = &prev_buttons - &buttons;
    if !new_buttons.is_empty() || !old_buttons.is_empty() {
        eprintln!(
            "X = {:?}, Y = {:?} : {:?} -> {:?}",
            mouse_state.x(),
            mouse_state.y(),
            new_buttons,
            old_buttons
        );
    }
    buttons
}

fn run(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    let texture_creator = canvas.texture_creator();
    let sprite = Sprite::load_from_file(
        "assets/17_mouse_events/button.png".as_ref(),
        &texture_creator,
    )?;
    let mut buttons = [
        Button::new(0, 0, BUTTON_WIDTH, BUTTON_HEIGHT, &sprite),
        Button::new(
            (WIDTH - BUTTON_WIDTH) as i32,
            0,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            &sprite,
        ),
        Button::new(
            0,
            (HEIGHT - BUTTON_HEIGHT) as i32,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            &sprite,
        ),
        Button::new(
            (WIDTH - BUTTON_WIDTH) as i32,
            (HEIGHT - BUTTON_HEIGHT) as i32,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            &sprite,
        ),
    ];

    let mut prev_buttons = HashSet::new();

    'running: loop {
        let mouse_state: MouseState = event_pump.mouse_state();
        if let Some(Event::Quit { .. }) = event_pump.poll_event() {
            break 'running;
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                event => {
                    for button in buttons.iter_mut() {
                        button.on_mouse_event(&event, &mouse_state);
                    }
                }
            }
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();
        for button in buttons.iter() {
            button.render(canvas)?;
        }
        canvas.present();

        // Logging mouse state whenever button state changed
        prev_buttons = log_mouse_button_state(&mouse_state, prev_buttons);
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
