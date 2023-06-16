use sdl2_wrapper::application::{Scene, Application, ApplicationBuilder, CanvasContext, Context};

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

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

        let target = app.target_texture_manager.load("target")?;
        app.canvas
            .with_texture_canvas(&mut target.as_ref().borrow_mut(), |texture_canvas| {
                texture_canvas.set_draw_color(Color::RGB(255, 0, 0));
                texture_canvas.clear();
                texture_canvas.set_draw_color(Color::RGB(0, 255, 0));
                texture_canvas.fill_rect(Rect::new(0, 0, 50, 100)).unwrap();
            })
            .map_err(|e| e.to_string())?;
        app.canvas
            .copy(&target.borrow(), None, Rect::new(0, 0, 100, 100))?;

        app.canvas.present();

        Ok(())
    }
}

fn main() -> Result<(), String> {
    let context = Context::new()?;
    let CanvasContext { canvas, texture_creator } = context.canvas_context()?;
    let mut app = ApplicationBuilder::new()
        .with_event_pump(context.event_pump())
        .with_canvas(canvas)
        .with_texture_creator(&texture_creator)
        .build()?;

    app.run(MyScene)
}
