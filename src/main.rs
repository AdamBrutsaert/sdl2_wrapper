use sdl2_wrapper::{window, application, Application, Context, Scene};

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
    let canvas_context = context.canvas_context(window!(context, "SDL2 Demo", 800, 600)?)?;
    let mut app = application!(context, canvas_context)?;

    app.run(MyScene)
}
