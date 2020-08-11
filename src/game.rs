use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;

use glium::glutin::event::{ElementState, Event, WindowEvent};

use glium::{Display, Program};

use std::sync::Mutex;
use std::time::Instant;

use crate::render::{Canvas, Model, VERTEX_SHADER, FRAGMENT_SHADER};

pub use glium::glutin::event::VirtualKeyCode as KeyCode;

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
    pressed: Vec<KeyCode>,
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
            pressed: Vec::new(),
            display,
            program,
            rect,
        }
    }

    fn new_canvas(&self) -> Canvas {
        Canvas::new(self.display.draw(), &self.program, &self.rect)
    }

    fn press(&mut self, key: KeyCode) {
        self.pressed.push(key);
    }

    fn release(&mut self, key: KeyCode) {
        let index = self.pressed.binary_search(&key);

        if let Ok(index) = index {
            self.pressed.remove(index);
        }
    }

    pub fn is_held(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn was_pressed(&mut self, key: KeyCode) -> bool {
        let held = self.is_held(key);

        if held {
            self.release(key);
            true
        } else {
            false
        }
    }
}

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

        context.delta = this_frame.duration_since(previous_frame).as_secs_f32();

        game.get_mut().unwrap().update(&mut context);

        let mut canvas = context.new_canvas();
        game.get_mut().unwrap().render(&mut canvas);
        canvas.finish();

        previous_frame = this_frame;
    });
}
