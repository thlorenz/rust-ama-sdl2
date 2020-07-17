# lazy foo rust

Reimplementing the great [Lazy Foo's Production SDL
Tutorial](https://lazyfoo.net/tutorials/SDL/) in Rust.

## Lessons

### 01 Hello SDL

- [tutorial](https://lazyfoo.net/tutorials/SDL/01_hello_SDL/index.php)

This one has no file associated as it's just an explanation on how to install SDL.

I recommend to also refer to instructions in the [main readme](../README.md).

### 02 Image On the Screen

```sh
cargo run --bin 02_image_on_screen
```

- [tutorial](https://lazyfoo.net/tutorials/SDL/02_getting_an_image_on_the_screen/index.php)
- [rust code](./src/02_image_on_screen.rs)
- [read up on high-dpi-mode](https://nlguillemot.wordpress.com/2016/12/11/high-dpi-rendering/) which is an option when creating the window

### 03 Event Driven Programming

```sh
cargo run --bin 03_event_driven
```

- [tutorial](http://lazyfoo.net/tutorials/SDL/03_event_driven_programming/index.php)
- [rust code](./src/03_event_driven.rs)

#### Note on the Event Polling Strategy

In the example we're using `event_pump.poll_event` since that is what the
 original tutorial is using. It returns the most recent event from the SDL event
  queue.
 It returns `0` when the queue is empty.
 Note that the original tutorial called this method in a loop until no more
  events are on the queue which is actually more akin to `event_poll.iter
  ` (see below). We don't do this as the game loop causes us to
   poll events on each iteration and here we're really only interested in the
    last one.

```rust
if let Some(Event::Quit { .. }) = event_pump.poll_event() {
    break 'running;
}
```

However, we could have used `event_poll.iter()` as well. It calls
 `even_pump.poll_event` repeatedly until there are no more events on the queue. This is a better
 approach in most situations, and we will use it going forward.

```rust
for event in event_pump.poll_iter() {
    match event {
        Event::Quit { .. } => break 'running,
        _ => {}
    }
}
```

### 04 Key Presses

```sh
cargo run --bin 04_key_presses
```

- [tutorial](http://lazyfoo.net/tutorials/SDL/04_key_presses/index.php)
- [rust code](./src/04_key_presses.rs)
- [SDK keycodes and scancodes](https://wiki.libsdl.org/SDL_Keycode)
- [alternate approach to query
  keyboard](https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/keyboard-state.rs) via
  `events.keyboard_state().pressed_scancodes()`

#### 05 Optimized Surface Loading and Soft Stretching

Modified to stretch to half screen size to prove that it is actually stretching.

```sh
cargo run --bin 05_optimized_loading_soft_stretching
```

 - [SDL_CreateTextureFromSurface](https://wiki.libsdl.org/SDL_CreateTextureFromSurface)
 - [SDL_ConvertSurface](https://wiki.libsdl.org/SDL_ConvertSurface)
 - [SDL_GetWindowPixelFormat](https://wiki.libsdl.org/SDL_GetWindowPixelFormat)
 - [SDL_PixelFormat](https://wiki.libsdl.org/SDL_PixelFormat)
 - [SDL_PixelFormatEnum](https://wiki.libsdl.org/SDL_PixelFormatEnum)
 - [SDL_UpdateWindowSurface](https://wiki.libsdl.org/SDL_UpdateWindowSurface)
 - [SDL_BlitScaled](https://wiki.libsdl.org/SDL_BlitScaled)
 - [WindowSurfaceRef::update_window](https://rust-sdl2.github.io/rust-sdl2/sdl2/video/struct.WindowSurfaceRef.html#method.update_window)

## Related Projects

- [rust-sdl2-lazyfoo](https://github.com/bombless/rust-sdl2-lazyfoo) from 2015 up to tutorial 18
- [rust-lazy-foo](https://github.com/ysgard/rust-lazy-foo) from 2016 up to tutorial 18
- [lazyfoo-rs](https://github.com/dagit/lazyfoo-rs) from 2017 tutorial 39 only
- [lazyfoo-sdl2-rs](https://github.com/mikeyhc/lazyfoo-sdl2-rs) from 2016 up to tutorial 5

## LICENSE

GPL2
