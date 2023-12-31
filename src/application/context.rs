use std::sync::{Arc, Mutex};

use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowBuilder, WindowContext};
use sdl2::EventPump;
use sdl2::VideoSubsystem;

pub struct Context {
    video_subsystem: VideoSubsystem,
    event_pump: Arc<Mutex<EventPump>>,
}

pub struct CanvasContext {
    pub canvas: Canvas<Window>,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl Context {
    pub fn new() -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;
        let event_pump = Arc::new(Mutex::new(sdl.event_pump()?));

        Ok(Self {
            video_subsystem,
            event_pump,
        })
    }

    pub fn event_pump(&self) -> Arc<Mutex<EventPump>> {
        self.event_pump.clone()
    }

    pub fn window(&self, title: &str, width: u32, height: u32) -> WindowBuilder {
        self.video_subsystem.window(title, width, height)
    }

    pub fn canvas_context(&self, window: Window) -> Result<CanvasContext, String> {
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

#[macro_export]
macro_rules! window {
    ($context:expr, $title:expr, $width:expr, $height:expr) => {
        {
            let window = $context
                .window($title, $width, $height)
                .position_centered()
                .build()
                .map_err(|e| e.to_string());
            window
        }
    };
}
