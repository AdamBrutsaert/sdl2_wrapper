pub mod context;
pub mod resource_loader;
pub mod resource_manager;

use std::sync::{Arc, Mutex};
use std::time::Instant;

pub use context::{CanvasContext, Context};
use resource_manager::TextureManager;

use sdl2::event::Event;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;

pub trait Scene {
    fn on_event(&mut self, app: &mut Application, event: Event) -> Result<(), String>;
    fn on_update(&mut self, app: &mut Application, dt: f64) -> Result<(), String>;
    fn on_render(&mut self, app: &mut Application) -> Result<(), String>;
}

pub struct Application<'a> {
    pub canvas: Canvas<Window>,
    event_pump: Arc<Mutex<EventPump>>,
    pub texture_manager: TextureManager<'a, WindowContext>,
}

impl Application<'_> {
    pub fn run(&mut self, mut scene: impl Scene) -> Result<(), String> {
        let event_pump = self.event_pump.clone();
        let mut before = Instant::now();

        'mainloop: loop {
            let current = Instant::now();
            let dt = current.duration_since(before).as_secs_f64();
            before = current;

            for event in event_pump.lock().unwrap().poll_iter() {
                match event {
                    Event::Quit { .. } => break 'mainloop,
                    _ => {}
                }

                scene.on_event(self, event)?;
            }
            scene.on_update(self, dt)?;
            scene.on_render(self)?;
        }

        Ok(())
    }
}

pub struct ApplicationBuilder<'a> {
    event_pump: Option<Arc<Mutex<EventPump>>>,
    canvas: Option<Canvas<Window>>,
    texture_creator: Option<&'a TextureCreator<WindowContext>>,
}

impl<'a> ApplicationBuilder<'a> {
    pub fn new() -> Self {
        Self {
            event_pump: None,
            canvas: None,
            texture_creator: None,
        }
    }

    pub fn build(self) -> Result<Application<'a>, String> {
        let event_pump = self.event_pump.ok_or("event_pump is not set")?;
        let canvas = self.canvas.ok_or("canvas is not set")?;
        let texture_creator = self.texture_creator.ok_or("texture_creator is not set")?;
        let texture_manager = TextureManager::new(texture_creator);

        Ok(Application {
            event_pump,
            canvas,
            texture_manager,
        })
    }

    pub fn with_event_pump(mut self, event_pump: Arc<Mutex<EventPump>>) -> Self {
        self.event_pump = Some(event_pump);
        self
    }

    pub fn with_canvas(mut self, canvas: Canvas<Window>) -> Self {
        self.canvas = Some(canvas);
        self
    }

    pub fn with_texture_creator(
        mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        self.texture_creator = Some(texture_creator);
        self
    }
}
