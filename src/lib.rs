#[macro_use]
extern crate glium;

use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;

use glium::glutin::event::Event;
use glium::glutin::event::WindowEvent;

use glium::index::PrimitiveType;

use glium::{Display, Program};

use std::sync::Mutex;
use std::time::Instant;

pub use glium::{Frame, Surface, SwapBuffersError};

pub struct GameConfig {
    title: String,
    width: u32,
    height: u32,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Vertex {
    fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }
}

type VertexBuffer = glium::VertexBuffer<Vertex>;
type IndexBuffer = glium::IndexBuffer<u16>;

pub struct Model {
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
}

impl Model {
    pub fn new(display: &Display, vertices: &[(f32, f32)], indices: &[u16]) -> Self {
        let vertices: Vec<_> = vertices
            .iter()
            .map(|points| Vertex::new(points.0, points.1))
            .collect();

        Self {
            vertex_buffer: VertexBuffer::new(display, &vertices).unwrap(),
            index_buffer: IndexBuffer::new(display, PrimitiveType::TriangleStrip, indices).unwrap(),
        }
    }
}

pub struct Canvas<'a> {
    frame: Frame,
    program: &'a Program,
    rect: &'a Model,
}

type Vec2 = [f32; 2];
type Color = [f32; 3];

impl<'a> Canvas<'a> {
    pub fn new(frame: Frame, program: &'a Program, rect: &'a Model) -> Self {
        Self {
            frame,
            program,
            rect,
        }
    }

    fn finish(self) {
        // TODO: handle errors.
        self.frame.finish().unwrap();
    }

    fn draw_model(&mut self, model: &Model, offset: Vec2, scale: Vec2, color: Color) {
        self.frame
            .draw(
                &model.vertex_buffer,
                &model.index_buffer,
                &self.program,
                &uniform! {
                    offset: offset,
                    scale: scale,
                    color: color,
                },
                &Default::default(),
            )
            .unwrap();
    }

    pub fn rect(&mut self, offset: Vec2, scale: Vec2, color: Color) {
        self.draw_model(self.rect, offset, scale, color);
    }

    pub fn rect_center(&mut self, offset: Vec2, scale: Vec2, color: Color) {
        self.rect(
            [offset[0] - scale[0] / 2.0, offset[1] - scale[1] / 2.0],
            scale,
            color,
        );
    }

    pub fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.frame.clear_color(r, g, b, a);
    }
}

pub trait Game {
    fn configure(&self, _config: &mut GameConfig) {}

    fn tick(&mut self, canvas: &mut Canvas, delta: f32);
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

    let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    let rect = Model::new(
        &display,
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        &[0, 1, 3, 2],
    );

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
        let delta = this_frame.duration_since(previous_frame).as_secs_f32();

        let mut canvas = Canvas::new(display.draw(), &program, &rect);
        game.get_mut().unwrap().tick(&mut canvas, delta);
        canvas.finish();

        previous_frame = this_frame;
    });
}
