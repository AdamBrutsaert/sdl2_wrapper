use sdl2_wrapper::application::{Application, ApplicationBuilder, Context, Scene};

use sdl2::event::Event;
use sdl2::pixels::Color;

struct MyScene;

impl Scene for MyScene {
    fn on_event(&mut self, _app: &mut Application, _event: Event) -> Result<(), String> {
        Ok(())
    }

    fn on_update(&mut self, _app: &mut Application, _dt: f64) -> Result<(), String> {
        Ok(())
    }

    fn on_render(&mut self, app: &mut Application) -> Result<(), String> {
        app.canvas.set_draw_color(Color::RGB(26, 26, 26));
        app.canvas.clear();

        let texture = app.texture_manager.load("assets/mario.png")?;
        app.canvas.copy(&texture, None, None)?;

        app.canvas.present();

        Ok(())
    }
}

fn main() -> Result<(), String> {
    let context = Context::new()?;

    let window = context
        .window("SDL2 Demo", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let canvas_context = context.canvas_context(window)?;

    let mut app = ApplicationBuilder::new()
        .with_event_pump(context.event_pump())
        .with_canvas(canvas_context.canvas)
        .with_texture_creator(&canvas_context.texture_creator)
        .build()?;

    app.run(MyScene)
}
