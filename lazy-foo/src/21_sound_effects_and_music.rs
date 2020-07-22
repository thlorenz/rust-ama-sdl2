use lazy_foo::{init_renderer, Sprite};
use sdl2::audio::{AudioCVT, AudioCallback, AudioDevice, AudioSpecDesired, AudioSpecWAV};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::error::Error;

// Mostly adapted from: https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-wav.rs
struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
        }
    }
}

fn load_audio(
    wav_file: &str,
    desired_spec: &AudioSpecDesired,
    sdl_context: &Sdl,
) -> Result<AudioDevice<Sound>, String> {
    let audio_subsystem = sdl_context.audio()?;
    audio_subsystem.open_playback(None, desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav(&wav_file).expect("FATAL: Could not load test WAV file");

        let cvt = AudioCVT::new(
            wav.format,
            wav.channels,
            wav.freq,
            spec.format,
            spec.channels,
            spec.freq,
        )
        .expect("FATAL: Could not convert WAV file");

        let data = cvt.convert(wav.buffer().to_vec());

        Sound {
            data,
            volume: 0.5,
            pos: 0,
        }
    })
}

fn run(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("FATAL: failed to init event_pump");

    let texture_creator = canvas.texture_creator();
    let sprite = Sprite::load_from_file(
        "assets/21_sound_effects_and_music/prompt.png".as_ref(),
        &texture_creator,
    )?;

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1), // mono
        samples: None,     // default
    };
    let beat = load_audio(
        "assets/21_sound_effects_and_music/beat.wav",
        &desired_spec,
        &sdl_context,
    )?;
    let high = load_audio(
        "assets/21_sound_effects_and_music/high.wav",
        &desired_spec,
        &sdl_context,
    )?;
    let low = load_audio(
        "assets/21_sound_effects_and_music/low.wav",
        &desired_spec,
        &sdl_context,
    )?;
    let medium = load_audio(
        "assets/21_sound_effects_and_music/medium.wav",
        &desired_spec,
        &sdl_context,
    )?;
    let scratch = load_audio(
        "assets/21_sound_effects_and_music/scratch.wav",
        &desired_spec,
        &sdl_context,
    )?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::Num1) => beat.resume(),
                    Some(Keycode::Num2) => high.resume(),
                    Some(Keycode::Num3) => low.resume(),
                    Some(Keycode::Num4) => medium.resume(),
                    Some(Keycode::Num5) => scratch.resume(),
                    _ => {}
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0xff));
        canvas.clear();
        sprite.render(canvas, 0, 0, None)?;
        canvas.present();
    }
    Ok(())
}

fn main() {
    let (sdl_context, mut canvas) =
        init_renderer().expect("FATAL: failed to initialize window and canvas.");
    run(&sdl_context, &mut canvas).expect("FATAL: something failed in the game loop");
}
