mod winsdl;

use std::time::Instant;
use sdl2::{event::Event, pixels::PixelFormatEnum};
use winsdl::Winsdl;


fn put_pixel(
    framebuffer: &mut [u8],
    width: usize,
    x: usize,
    y: usize,
    r: u8,
    g: u8,
    b: u8,
) {
    let index = (y * width + x) * 3;
    framebuffer[index] = r;
    framebuffer[index + 1] = g;
    framebuffer[index + 2] = b;
}


fn main() {
    let mut winsdl = Winsdl::new(800, 600).unwrap();
    let mut framebuffer = vec![0u8; winsdl.width * winsdl.height * 3];

    let texture_creator = winsdl.canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, winsdl.width as u32, winsdl.height as u32)
        .unwrap();

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            if let Event::Quit { .. } = event { break 'running; }
        }

        // --- DRAWING START ---
        for y in 0..winsdl.height {
            for x in 0..winsdl.width {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                if (x < winsdl.width / 2){
                    b = 255;
                }

                put_pixel(&mut framebuffer, winsdl.width, x, y, r, g, b);
            }
        }
        
        // --- DRAWING END ---
        texture.update(None, &framebuffer, winsdl.width * 3).unwrap();

        winsdl.canvas.clear();
        winsdl.canvas.copy(&texture, None, None).unwrap();
        winsdl.canvas.present();
    }
}


