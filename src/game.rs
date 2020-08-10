use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;

use glium::glutin::event::Event;
use glium::glutin::event::WindowEvent;

use glium::{Display, Program};

use std::sync::Mutex;
use std::time::Instant;

use crate::render::{Canvas, Model};

pub struct GameConfig {
    pub title: String,
    pub window_size: (u32, u32),
}

pub trait Game {
    fn configure(&self, _config: &mut GameConfig) {}
    fn render(&mut self, canvas: &mut Canvas);
    fn update(&mut self, context: &mut Context);
}

pub struct Context {
    pub delta: f32,
    display: Display,
    program: Program,
    rect: Model,
}

type EventLoop = glium::glutin::event_loop::EventLoop<()>;

impl Context {
    fn new(config: GameConfig, event_loop: &EventLoop) -> Self {
        let (width, height) = config.window_size;

        let window_builder = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(width, height))
            .with_title(config.title);

        let context_builder = ContextBuilder::new().with_vsync(true);

        let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

        let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

        let rect = Model::new(
            &display,
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            &[0, 1, 3, 2],
        );

        Self {
            delta: 0.0,
            display,
            program,
            rect,
        }
    }

    fn new_canvas(&self) -> Canvas {
        Canvas::new(self.display.draw(), &self.program, &self.rect)
    }
}

static VERTEX_SHADER: &str = "
#version 140

uniform vec2 offset;
uniform vec2 scale;
uniform vec3 color;

in vec2 position;

out vec3 vColor;

void main() {
    gl_Position = vec4(offset + position * scale, 0.0, 1.0);
    vColor = color;
}
";

static FRAGMENT_SHADER: &str = "
#version 140

in vec3 vColor;

out vec4 f_color;

void main() {
    f_color = vec4(vColor, 1.0);
}
";

pub fn run_game<T: 'static + Game>(game: T) {
    let mut game = Mutex::new(game);

    let mut config = GameConfig {
        title: "My Game".into(),
        window_size: (640, 420),
    };

    game.get_mut().unwrap().configure(&mut config);

    let event_loop = EventLoop::new();

    let mut context = Context::new(config, &event_loop);

    let mut previous_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Some(event) = event.to_static() {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let this_frame = Instant::now();

        context.delta = this_frame.duration_since(previous_frame).as_secs_f32();

        game.get_mut().unwrap().update(&mut context);

        let mut canvas = context.new_canvas();
        game.get_mut().unwrap().render(&mut canvas);
        canvas.finish();

        previous_frame = this_frame;
    });
}
