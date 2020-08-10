use glium::index::PrimitiveType;

use glium::{Display, Frame, Program, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: (f32, f32),
}

implement_vertex!(Vertex, position);

impl Vertex {
    fn new(x: f32, y: f32) -> Self {
        Self { position: (x, y) }
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

type Vec2 = (f32, f32);
type Color = (f32, f32, f32);

impl<'a> Canvas<'a> {
    pub fn new(frame: Frame, program: &'a Program, rect: &'a Model) -> Self {
        Self {
            frame,
            program,
            rect,
        }
    }

    pub fn finish(self) {
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
        let (x, y) = offset;
        let (w, h) = scale;

        self.rect((x - w / 2.0, y - h / 2.0), scale, color);
    }

    pub fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.frame.clear_color(r, g, b, a);
    }
}
