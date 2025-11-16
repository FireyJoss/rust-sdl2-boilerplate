use sdl2::{
    Sdl,
    EventPump,
    video::Window,
    render::Canvas,
};

pub struct Winsdl {
    pub sdl: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub width: usize,
    pub height: usize,
}

impl Winsdl {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window("Renderer3D", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        let event_pump = sdl.event_pump().unwrap();

        Ok(Winsdl {
            sdl,
            canvas,
            event_pump,
            width,
            height,
        })
    }
}
