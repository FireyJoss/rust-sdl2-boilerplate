Rust SDL2 Boilerplate

A minimal Rust project template that creates an SDL2 window and provides a simple CPU-based pixel framebuffer.
Use this as a starting point for software renderers, raycasters, raytracers, voxel engines, and other low-level graphics experiments.

Features

- SDL2 window initialization
- Streaming texture for fast pixel updates
- Simple put_pixel() helper function
- CPU-rendered framebuffer
- Clean structure for extending into larger renderer projects

Requirements

- Rust
- SDL2 development libraries
  Linux (Debian/Ubuntu)
    - sudo apt install libsdl2-dev
  macOS (Homebrew)
    - brew install sdl2
  Windows
  Install through:
    - MSYS2: pacman -S mingw-w64-x86_64-SDL2
    - vcpkg
    - download prebuilt binaries from the SDL website

Running the Project
  cargo run

How to Draw Pixels
This helper function writes an RGB pixel into the framebuffer:
put_pixel(&mut framebuffer, width, x, y, r, g, b);

Project Structure
src/
├── main.rs      # Main loop, framebuffer, texture update
└── winsdl.rs    # SDL2 window wrapper (Winsdl struct)


