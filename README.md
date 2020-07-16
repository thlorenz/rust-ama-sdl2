# rust-ama-sdl2

Several examples and project demonstrating how to use SDL2 with Rust.

## Requirements

### Rust

[Install Rust](http://www.rust-lang.org/install.html) via the following steps.

Get `rustup`:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

and then do the below to install and use the stable Rust version:

```sh
rustup toolchain install stable
rustup default stable
```

### SDL

Install the following:

- [SDL2 Development libraries](https://www.libsdl.org/download-2.0.php)
- [SDL_Image 2.0](https://www.libsdl.org/projects/SDL_image/) image library
- [SDL_TTF 2.0](https://www.libsdl.org/projects/SDL_ttf/) truetype library

#### OSX using Homebrew

```sh
brew install sdl2
brew install sdl2_image
brew install sdl2_ttf
```

#### Fedora:

```sh
sudo dnf install SDL2-devel SDL2_ttf-devel SDL2_image-devel
```
    
#### RedHat/Centos:

```sh
sudo yum install SDL2-devel SDL2_ttf-devel SDL2_image-devel
```

## LICENSE

GPL2
