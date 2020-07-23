use one_offs::{init_renderer, Sprite};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::Sdl;
use std::error::Error;

const TILES_WIDTH: u32 = 512;
const TILES_HEIGHT: u32 = 512;
const TILES_PER_ROW: u32 = 8;
const TILES_PER_COL: u32 = 8;
const TILE_WIDTH: u32 = TILES_WIDTH / TILES_PER_COL;
const TILE_HEIGHT: u32 = TILES_HEIGHT / TILES_PER_ROW;

fn get_clip(col: u32, row: u32) -> Rect {
    Rect::new(
        (col * TILE_WIDTH) as i32,
        (row * TILE_HEIGHT) as i32,
        TILE_WIDTH,
        TILE_HEIGHT,
    )
}

struct Tile {
    pub position: Point,
    pub clip: Rect,
}

fn render_tiles(
    canvas: &mut WindowCanvas,
    floor_tiles_sprite: &Sprite,
    tiles: &Vec<Tile>,
    offset: &Point,
    render_all: bool,
) -> Result<(), String> {
    let (width, height) = canvas.window().size();
    let padded_window_rect = Rect::new(
        -(TILE_WIDTH as i32),
        -(TILE_HEIGHT as i32),
        width + TILE_WIDTH,
        height + TILE_HEIGHT,
    );
    for tile in tiles {
        let x = tile.position.x - offset.x;
        let y = tile.position.y - offset.y;
        if render_all || padded_window_rect.contains_point(Point::new(x, y)) {
            floor_tiles_sprite.render(canvas, x, y, tile.clip)?;
        }
    }
    Ok(())
}

fn render_texture(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    offset: &Point,
) -> Result<(), String> {
    let (width, height) = canvas.window().size();
    let src = Rect::new(offset.x, offset.y, width, height);
    canvas.copy(texture, src, None)
}

fn run(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    let texture_creator = canvas.texture_creator();
    let floor_tiles_sprite =
        Sprite::load_from_file("assets/floor-tiles.png".as_ref(), &texture_creator)?;

    let mut clip_row = 0;
    let mut clip_col = 0;
    let mut tiles: Vec<Tile> = Vec::new();
    let mut speed = Point::new(0, 0);
    let mut offset = Point::new(100 * TILE_WIDTH as i32, 100 * TILE_HEIGHT as i32);

    eprintln!("Use WASD to move the tiles and Space to render using texture target to see 4-5x drop in CPU usage.");

    for row in 0..200 {
        for col in 0..200 {
            let position = Point::new((col * TILE_WIDTH) as i32, (row * TILE_HEIGHT) as i32);
            let clip = get_clip(clip_col, clip_row);
            tiles.push(Tile { position, clip });
            clip_col += 1;
            if clip_col == TILES_PER_COL {
                clip_col = 0;
                clip_row += 1;
            };
            if clip_row == TILES_PER_ROW {
                clip_row = 0
            };
        }
    }

    let texture_width = 200 * TILE_WIDTH;
    let texture_height = 200 * TILE_HEIGHT;
    let mut tgt_texture = texture_creator.create_texture_target(
        canvas.default_pixel_format(),
        texture_width,
        texture_height,
    )?;
    let mut use_texture: bool = false;

    canvas.with_texture_canvas(&mut tgt_texture, |texture_canvas| {
        render_tiles(
            texture_canvas,
            &floor_tiles_sprite,
            &tiles,
            &Point::new(0, 0),
            true,
        )
        .expect("FATAL: failed to render to target texture");
    })?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::W) => speed.y -= 1,
                    Some(Keycode::S) => speed.y += 1,
                    Some(Keycode::A) => speed.x -= 1,
                    Some(Keycode::D) => speed.x += 1,
                    Some(Keycode::Space) => {
                        use_texture = !use_texture;
                        eprintln!("using texture {}", use_texture);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();
        if use_texture {
            render_texture(canvas, &tgt_texture, &offset)?;
        } else {
            render_tiles(canvas, &floor_tiles_sprite, &tiles, &offset, false)?;
        }
        canvas.present();

        offset.x += speed.x;
        offset.y += speed.y;
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
