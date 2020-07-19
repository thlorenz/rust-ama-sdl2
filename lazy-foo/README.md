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

#### 06 Loading PNGs with SDL_image

```sh
cargo run --bin 06_loading_pngs
```

Note that I got this warning initially:
> // libpng warning: iCCP: known incorrect sRGB profile

which was easily fixed via: `convert loaded.png fixed.png` and using the
 _fixed_ png instead which also turned out to be half the size.

- [tutorials installation
  instructions](http://lazyfoo.net/tutorials/SDL/06_extension_libraries_and_loading_other_image_formats/index.php)
  - not needed if you followed the [main readme](../README.md) instructions
- [tutorial](http://lazyfoo.net/tutorials/SDL/06_extension_libraries_and_loading_other_image_formats/index2.php)
- [rust-sdl2 load_texture](https://rust-sdl2.github.io/rust-sdl2/sdl2/render
/struct.TextureCreator.html#method.load_texture)
- [rust-sdl2 image demo](https://github.com/Rust-SDL2/rust-sdl2/blob/master
/examples/image-demo.rs)
- [SDL2_Image docs](https://www.libsdl.org/projects/SDL_image/docs
/SDL_image_frame.html)
  - [`SDL_Surface *IMG_Load(const char *file)`](https://www.libsdl.org/projects/SDL_image/docs/SDL_image_frame.html)

#### 07 Texture Loading and Rendering

```sh
cargo run --bin 07_textures
```

This tutorial is introducing an API which we've been using already as that is what most
examples are suggesting at this point, at least for rust-sdl2.

- [tutorial](http://lazyfoo.net/tutorials/SDL/07_texture_loading_and_rendering/index.php)
- [SDL_TEXTURE](https://wiki.libsdl.org/SDL_Texture)

#### 08 Geometry Rendering

```sh
cargo run --bin 08_geometry
```

- [tutorial](http://lazyfoo.net/tutorials/SDL/08_geometry_rendering/index.php)
- [SDL2 Rendering API](https://wiki.libsdl.org/CategoryRender)
- [SDL_RECT](https://wiki.libsdl.org/SDL_Rect)
- [SDL_RenderFillRect](https://wiki.libsdl.org/SDL_RenderFillRect)
- [SDL_RenderDrawRect](https://wiki.libsdl.org/SDL_RenderDrawRect)

#### 09 The Viewport

```sh
cargo run --bin 09_viewport
```

NOTE: needed to fix png here as well via: `convert viewport.png viewport.png` in order to avoid
libpng warning.

- [tutorial](http://lazyfoo.net/tutorials/SDL/09_the_viewport/index.php)
- [SDL_RenderSetViewport](https://wiki.libsdl.org/SDL_RenderSetViewport)
- [explanation of libpng incorrect sRGP profile warning](https://stackoverflow.com/a/22747902)

#### 10 Color Keying

```sh
cargo run --bin 10_color_keying
```

NOTE: running this example drove CPU usage of _WindowServer_ on macOS up, most likely due to
applying transparency.

- [tutorial](http://lazyfoo.net/tutorials/SDL/10_color_keying/index.php)
- [sdl2::image::LoadSurface::from_file](http://rust-sdl2.github.io/rust-sdl2/sdl2/image/trait.LoadSurface.html#tymethod.from_file)
- [SDL_SetColorKey](https://wiki.libsdl.org/SDL_SetColorKey)
- [sdl2::Surface::set_color_key](https://docs.rs/sdl2/0.34.2/sdl2/surface/struct.SurfaceRef.html#method.set_color_key)
- [SDL_MapRGB](https://wiki.libsdl.org/SDL_MapRGB)

#### 11 Clip Rendering and Sprite Sheets

```sh
cargo run --bin 11_clip_rendering
```

- [tutorial](http://lazyfoo.net/tutorials/SDL/11_clip_rendering_and_sprite_sheets/index.php)

## Related Projects

- [rust-sdl2-lazyfoo](https://github.com/bombless/rust-sdl2-lazyfoo) from 2015 up to tutorial 18
- [rust-lazy-foo](https://github.com/ysgard/rust-lazy-foo) from 2016 up to tutorial 18
- [lazyfoo-rs](https://github.com/dagit/lazyfoo-rs) from 2017 tutorial 39 only
- [lazyfoo-sdl2-rs](https://github.com/mikeyhc/lazyfoo-sdl2-rs) from 2016 up to tutorial 5

#### 12 Color Modulation

```sh
cargo run --bin 12_color_modulation
```

Way to multiply a color throughout the whole texture.
For `RGB(255, 128, 255)` it halves the green component for any pixel on the texture.

- [tutorial](http://lazyfoo.net/tutorials/SDL/12_color_modulation/index.php)
- [SDL_SetTextureColorMod](https://wiki.libsdl.org/SDL_SetTextureColorMod)

#### 13 Alpha Blending

```sh
cargo run --bin 13_alpha_blending
```

- [tutorial](http://lazyfoo.net/tutorials/SDL/13_alpha_blending/index.php)
- [SDL_BlendMode](https://wiki.libsdl.org/SDL_BlendMode)
- [Blend Modes](https://en.wikipedia.org/wiki/Blend_modes)

#### 14 Animated Sprites and VSync

```sh
cargo run --bin 14_animated_sprites
```

- [tutorial](http://lazyfoo.net/tutorials/SDL/14_animated_sprites_and_vsync/index.php)


## LICENSE

GPL2
