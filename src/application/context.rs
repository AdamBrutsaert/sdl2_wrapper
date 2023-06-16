use std::cell::RefCell;
use std::rc::Rc;

use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use sdl2::VideoSubsystem;

pub struct Context {
    video_subsystem: VideoSubsystem,
    event_pump: Rc<RefCell<EventPump>>,
}

pub struct CanvasContext {
    pub canvas: Canvas<Window>,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl Context {
    pub fn new() -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;
        let event_pump = Rc::new(RefCell::new(sdl.event_pump()?));

        Ok(Self {
            video_subsystem,
            event_pump,
        })
    }

    pub fn event_pump(&self) -> Rc<RefCell<EventPump>> {
        self.event_pump.clone()
    }

    pub fn canvas_context(&self) -> Result<CanvasContext, String> {
        let window = self
            .video_subsystem
            .window("SDL2", 640, 480)
            .opengl()
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .target_texture()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();

        Ok(CanvasContext {
            canvas,
            texture_creator,
        })
    }
}
