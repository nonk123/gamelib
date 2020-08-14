use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;

use glium::glutin::event::{ElementState, Event, WindowEvent};

use glium::uniforms::{AsUniformValue, UniformValue};

use glium::{Display, Program};

use std::collections::{HashMap, HashSet};
use std::ops::Mul;
use std::sync::Mutex;
use std::time::Instant;

use crate::render::{Canvas, Model, ModelRenderBuilder, FRAGMENT_SHADER, VERTEX_SHADER};

pub use glium::glutin::event::VirtualKeyCode as KeyCode;

pub type Vec2 = (f32, f32);
pub type Color = (f32, f32, f32);

#[derive(Copy, Clone)]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = self;

        for i in 0..4 {
            for j in 0..4 {
                result.0[i][j] = 0.0;

                for n in 0..4 {
                    result.0[i][j] += self.0[i][n] * other.0[n][j];
                }
            }
        }

        result
    }
}

impl AsUniformValue for Mat4 {
    fn as_uniform_value(&self) -> UniformValue {
        AsUniformValue::as_uniform_value(&self.0)
    }
}

pub struct GameConfig {
    pub title: String,
    pub window_size: (u32, u32),
    pub update_fps: f32,
}

pub trait Game {
    fn configure(&self, _config: &mut GameConfig) {}
    fn init(&mut self, _context: &mut Context) {}
    fn render(&mut self, _canvas: &mut Canvas, _context: &mut Context) {}
    fn update(&mut self, _context: &mut Context) {}
}

pub struct Context {
    pub delta: f32,
    models: HashMap<String, Model>,
    pressed: HashSet<KeyCode>,
    display: Display,
}

type EventLoop = glium::glutin::event_loop::EventLoop<()>;

impl Context {
    fn new(config: &GameConfig, event_loop: &EventLoop) -> Self {
        let (width, height) = config.window_size;

        let window_builder = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(width, height))
            .with_title(config.title.to_string());

        let context_builder = ContextBuilder::new();

        let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

        Self {
            delta: 0.0,
            models: HashMap::new(),
            pressed: HashSet::new(),
            display,
        }
    }

    fn press(&mut self, key: KeyCode) {
        self.pressed.insert(key);
    }

    fn release(&mut self, key: KeyCode) {
        self.pressed.remove(&key);
    }

    pub fn is_held(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn was_pressed(&mut self, key: KeyCode) -> bool {
        let held = self.is_held(key);
        self.release(key);
        held
    }

    pub fn get_sprite(&mut self, filename: &str) -> &Model {
        if self.models.contains_key(&filename.to_string()) {
            return self.models.get(&filename.to_string()).unwrap();
        }

        let model = Model::square(
            &self.display,
            if filename == "rect" {
                None
            } else {
                Some(filename)
            },
        );

        self.models.insert(filename.to_string(), model);
        self.models.get(&filename.to_string()).unwrap()
    }

    pub fn render(&mut self, filename: &str) -> ModelRenderBuilder {
        ModelRenderBuilder::new(self.get_sprite(filename))
    }
}

pub fn run_game<T: 'static + Game>(game: T) {
    let mut game = Mutex::new(game);

    let mut config = GameConfig {
        title: "My Game".into(),
        window_size: (640, 420),
        update_fps: 24.0,
    };

    game.get_mut().unwrap().configure(&mut config);

    let event_loop = EventLoop::new();

    let mut context = Context::new(&config, &event_loop);
    context.delta = 1.0 / config.update_fps;

    let program =
        Program::from_source(&context.display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    game.get_mut().unwrap().init(&mut context);

    let mut previous_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Some(event) = event.to_static() {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            match input.state {
                                ElementState::Pressed => context.press(key),
                                ElementState::Released => context.release(key),
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let this_frame = Instant::now();
        let mut catchup = this_frame.duration_since(previous_frame).as_secs_f32();

        while catchup > context.delta {
            game.get_mut().unwrap().update(&mut context);
            catchup -= context.delta;
            previous_frame = this_frame;
        }

        let mut canvas = Canvas::new(context.display.draw(), &program);
        game.get_mut().unwrap().render(&mut canvas, &mut context);
        canvas.finish();
    });
}
