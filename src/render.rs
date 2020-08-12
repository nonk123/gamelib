use glium::index::PrimitiveType;

use glium::texture::{
    CompressedMipmapsOption, CompressedSrgbFormat, CompressedSrgbTexture2d, RawImage2d,
};

use glium::{Display, DrawParameters, Frame, Program, Rect, Surface};

use std::cmp;

#[derive(Copy, Clone)]
struct Vertex {
    position: (f32, f32),
    tex_coords: (f32, f32),
}

implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    fn new(x: f32, y: f32, tx: f32, ty: f32) -> Self {
        Self {
            position: (x, y),
            tex_coords: (tx, ty),
        }
    }
}

type VertexBuffer = glium::VertexBuffer<Vertex>;
type IndexBuffer = glium::IndexBuffer<u16>;

pub struct Model {
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    texture: CompressedSrgbTexture2d,
}

impl Model {
    pub fn new(
        display: &Display,
        vertices: &[(f32, f32, f32, f32)],
        indices: &[u16],
        texture: Option<&str>,
    ) -> Self {
        let vertices: Vec<_> = vertices
            .iter()
            .map(|points| Vertex::new(points.0, points.1, points.2, points.3))
            .collect();

        Self {
            vertex_buffer: VertexBuffer::new(display, &vertices).unwrap(),
            index_buffer: IndexBuffer::new(display, PrimitiveType::TriangleStrip, indices).unwrap(),
            texture: match texture {
                Some(filename) => {
                    let image = image::open(&filename)
                        .expect(format!("Couldn't load image {}", filename).as_str())
                        .to_rgba();

                    let dimensions = image.dimensions();

                    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);

                    CompressedSrgbTexture2d::new(display, image).unwrap()
                }
                None => CompressedSrgbTexture2d::empty_with_format(
                    display,
                    CompressedSrgbFormat::S3tcDxt1Alpha,
                    CompressedMipmapsOption::NoMipmap,
                    1,
                    1,
                )
                .unwrap(),
            },
        }
    }
}

enum ViewportScaling {
    Stretch,
    Fit,
}

pub struct Viewport {
    size: (f32, f32),
    scaling: ViewportScaling,
}

impl Viewport {
    fn new(size: (f32, f32), scaling: ViewportScaling) -> Self {
        Self { size, scaling }
    }

    pub fn stretch(size: (f32, f32)) -> Self {
        Self::new(size, ViewportScaling::Stretch)
    }

    pub fn fit(size: (f32, f32)) -> Self {
        Self::new(size, ViewportScaling::Fit)
    }

    fn get_dimensions(&self, frame: &Frame) -> Rect {
        let screen_size = frame.get_dimensions();

        let smallest_side = cmp::min(screen_size.0, screen_size.1) as f32;

        let screen_size = (screen_size.0 as f32, screen_size.1 as f32);

        let screen_ar = screen_size.0 / screen_size.1;
        let expected_ar = self.size.0 / self.size.1;

        let expected_size = if screen_ar > expected_ar {
            (smallest_side * expected_ar, smallest_side)
        } else {
            (smallest_side, smallest_side * expected_ar)
        };

        let position: (u32, u32) = match self.scaling {
            ViewportScaling::Stretch => (0, 0),
            ViewportScaling::Fit => {
                let mut h_fringe = 0.0;
                let mut v_fringe = 0.0;

                if screen_ar > expected_ar {
                    h_fringe = (screen_size.0 - expected_size.0) / 2.0;
                } else if screen_ar < expected_ar {
                    v_fringe = (screen_size.1 - expected_size.1) / 2.0;
                }

                (h_fringe as u32, v_fringe as u32)
            }
        };

        Rect {
            left: position.0,
            bottom: position.1,
            width: expected_size.0 as u32,
            height: expected_size.1 as u32,
        }
    }
}

pub struct Camera {
    position: Vec2,
}

impl Camera {
    fn new() -> Self {
        Self {
            position: (0.0, 0.0),
        }
    }
}

pub struct Canvas<'a> {
    frame: Frame,
    program: &'a Program,
    viewport: Viewport,
    camera: Camera,
}

type Vec2 = (f32, f32);
type Color = (f32, f32, f32);

impl<'a> Canvas<'a> {
    pub fn new(frame: Frame, program: &'a Program) -> Self {
        Self {
            viewport: Viewport::stretch((1.0, 1.0)),
            camera: Camera::new(),
            frame,
            program,
        }
    }

    pub fn finish(self) {
        // TODO: handle errors.
        self.frame.finish().unwrap();
    }

    pub fn stretch(&mut self, width: f32, height: f32) {
        self.viewport = Viewport::stretch((width, height));
    }

    pub fn fit(&mut self, width: f32, height: f32) {
        self.viewport = Viewport::fit((width, height));
    }

    pub fn model(&mut self, model: &Model, offset: Vec2, scale: Vec2, color: Color) {
        let mut parameters = DrawParameters::default();
        parameters.viewport = Some(self.viewport.get_dimensions(&self.frame));

        let mult = |a: [[f32; 4]; 4], b: [[f32; 4]; 4]| {
            let mut c = a;

            for i in 0..4 {
                for j in 0..4 {
                    c[i][j] = 0.0;

                    for n in 0..4 {
                        c[i][j] += a[i][n] * b[n][j];
                    }
                }
            }

            c
        };

        let view_matrix = [
            [1.0 / self.viewport.size.0, 0.0, 0.0, self.camera.position.0],
            [0.0, 1.0 / self.viewport.size.1, 0.0, self.camera.position.1],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let model_matrix = [
            [scale.0, 0.0, 0.0, offset.0],
            [0.0, scale.1, 0.0, offset.1],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        self.frame
            .draw(
                &model.vertex_buffer,
                &model.index_buffer,
                &self.program,
                &uniform! {
                    mvp: mult(view_matrix, model_matrix),
                    color: color,
                    tex: &model.texture,
                },
                &parameters,
            )
            .unwrap();
    }

    pub fn clear(&mut self, r: f32, g: f32, b: f32) {
        self.frame.clear_color(r, g, b, 1.0);
    }
}

pub const VERTEX_SHADER: &str = "
#version 140

uniform mat4 mvp;
uniform vec3 color;

in vec2 position;
in vec2 tex_coords;

out vec3 v_color;
out vec2 v_tex_coords;

void main() {
    gl_Position = vec4(position, 0.0, 1.0) * mvp;
    v_color = color;
    v_tex_coords = tex_coords;
}
";

pub const FRAGMENT_SHADER: &str = "
#version 140

uniform sampler2D tex;

in vec3 v_color;
in vec2 v_tex_coords;

out vec4 f_color;

void main() {
    // Use solid color instead of dummy 1x1 texture.
    if (textureSize(tex, 0) == vec2(1, 1)) {
        f_color = vec4(v_color, 1.0);
    } else {
        vec4 t_color = texture(tex, v_tex_coords);
        f_color = mix(t_color, vec4(v_color, 1.0), 0.5);
    }
}
";
