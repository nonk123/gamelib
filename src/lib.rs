use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;

use glium::glutin::event::Event;
use glium::glutin::event::WindowEvent;

use glium::Display;

use std::sync::Mutex;
use std::time::Instant;

pub use glium::{Frame, SwapBuffersError};

pub struct GameConfig {
    title: String,
    width: u32,
    height: u32,
}

pub struct Canvas {
    frame: Frame,
}

impl Canvas {
    pub fn new(frame: Frame) -> Self {
        Self { frame }
    }

    fn finish(self) {
        // TOOD: handle errors.
        self.frame.finish().unwrap();
    }

    pub fn rect(&mut self, _x: f32, _y: f32, _w: f32, _h: f32) {
        // TODO: implement.
    }

    pub fn rect_center(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.rect(x - w / 2.0, y - h / 2.0, w, h);
    }
}

pub trait Game {
    fn configure(&self, _config: &mut GameConfig) {}

    fn tick(&mut self, canvas: &mut Canvas, delta: f32);
}

pub fn run_game<T: 'static + Game>(game: T) {
    let mut game = Mutex::new(game);

    let mut config = GameConfig {
        title: "My Game".into(),
        width: 640,
        height: 420,
    };

    game.get_mut().unwrap().configure(&mut config);

    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(config.width, config.height))
        .with_title(config.title);

    let context_builder = ContextBuilder::new().with_vsync(true);

    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

    let mut previous_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Some(event) = event.to_static() {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let this_frame = Instant::now();
        let delta = this_frame.duration_since(previous_frame).as_secs_f32();

        let mut canvas = Canvas::new(display.draw());
        game.get_mut().unwrap().tick(&mut canvas, delta);
        canvas.finish();

        previous_frame = this_frame;
    });
}
