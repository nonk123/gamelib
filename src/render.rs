use glium::index::PrimitiveType;

use glium::texture::{
    CompressedMipmapsOption, CompressedSrgbFormat, CompressedSrgbTexture2d, RawImage2d,
};

use glium::{Display, DrawParameters, Frame, Program, Rect, Surface};

use std::cmp;

use crate::game::{Color, Mat4, Vec2};

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

    /// Mostly for internal use; `Context::get_sprite` manages these for you.
    pub fn square(display: &Display, texture: Option<&str>) -> Self {
        Self::new(
            display,
            &[
                (-0.5, -0.5, 0.0, 0.0),
                (0.5, -0.5, 1.0, 0.0),
                (0.5, 0.5, 1.0, 1.0),
                (-0.5, 0.5, 0.0, 1.0),
            ],
            &[0, 1, 3, 2],
            texture,
        )
    }
}

enum ViewportScaling {
    Stretch,
    Fit,
}

pub struct Viewport {
    width: f32,
    height: f32,
    scaling: ViewportScaling,
}

impl Viewport {
    fn new(width: f32, height: f32, scaling: ViewportScaling) -> Self {
        Self {
            width,
            height,
            scaling,
        }
    }

    pub fn stretch(width: f32, height: f32) -> Self {
        Self::new(width, height, ViewportScaling::Stretch)
    }

    pub fn fit(width: f32, height: f32) -> Self {
        Self::new(width, height, ViewportScaling::Fit)
    }

    fn get_dimensions(&self, frame: &Frame) -> Rect {
        let screen_size = frame.get_dimensions();

        let smallest_side = cmp::min(screen_size.0, screen_size.1) as f32;

        let screen_size = (screen_size.0 as f32, screen_size.1 as f32);

        let screen_ar = screen_size.0 / screen_size.1;
        let expected_ar = self.width / self.height;

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
    x: f32,
    y: f32,
}

impl Camera {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

pub struct ModelRenderBuilder<'a> {
    model: &'a Model,
    position: Vec2,
    scale: Vec2,
    rotation: f32,
    color: Color,
}

impl<'a> ModelRenderBuilder<'a> {
    pub fn new(model: &'a Model) -> Self {
        Self {
            model,
            position: (0.0, 0.0),
            scale: (1.0, 1.0),
            rotation: 0.0,
            color: (0.0, 0.0, 0.0),
        }
    }

    pub fn translate(mut self, dx: f32, dy: f32) -> Self {
        self.position.0 += dx;
        self.position.1 += dy;
        self
    }

    pub fn translate_tup(self, (dx, dy): (f32, f32)) -> Self {
        self.translate(dx, dy)
    }

    pub fn scale(mut self, x_mult: f32, y_mult: f32) -> Self {
        self.scale.0 *= x_mult;
        self.scale.1 *= y_mult;
        self
    }

    pub fn scale_tup(self, (x_mult, y_mult): (f32, f32)) -> Self {
        self.scale(x_mult, y_mult)
    }

    pub fn rotate(mut self, by_rad: f32) -> Self {
        self.rotation += by_rad;
        self
    }

    pub fn shade(mut self, red: f32, green: f32, blue: f32) -> Self {
        self.color = (red, green, blue);
        self
    }

    pub fn shade_tup(self, (red, green, blue): (f32, f32, f32)) -> Self {
        self.shade(red, green, blue)
    }

    pub fn commit(self, canvas: &mut Canvas) {
        canvas.render_model_from_builder(self);
    }

    fn get_model_matrix(&self) -> Mat4 {
        let translation = Mat4([
            [1.0, 0.0, 0.0, self.position.0],
            [0.0, 1.0, 0.0, self.position.1],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let rotation = Mat4([
            [self.rotation.cos(), -self.rotation.sin(), 0.0, 0.0],
            [self.rotation.sin(), self.rotation.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let scale = Mat4([
            [self.scale.0, 0.0, 0.0, 0.0],
            [0.0, self.scale.1, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        translation * rotation * scale
    }
}

pub struct Canvas<'a> {
    frame: Frame,
    program: &'a Program,
    viewport: Viewport,
    camera: Camera,
}

impl<'a> Canvas<'a> {
    pub fn new(frame: Frame, program: &'a Program) -> Self {
        Self {
            viewport: Viewport::stretch(1.0, 1.0),
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
        self.viewport = Viewport::stretch(width, height);
    }

    pub fn fit(&mut self, width: f32, height: f32) {
        self.viewport = Viewport::fit(width, height);
    }

    pub fn look_at(&mut self, x: f32, y: f32) {
        self.camera.x = x;
        self.camera.y = y;
    }

    pub fn render_model_from_builder(&mut self, renderer: ModelRenderBuilder) {
        let mut parameters = DrawParameters::default();
        parameters.viewport = Some(self.viewport.get_dimensions(&self.frame));

        // Scale to match the viewport's size.
        let projection_matrix = Mat4([
            [1.0 / self.viewport.width, 0.0, 0.0, 0.0],
            [0.0, 1.0 / self.viewport.height, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        // Translate away from camera.
        let view_matrix = Mat4([
            [1.0, 0.0, 0.0, -self.camera.x],
            [0.0, 1.0, 0.0, -self.camera.y],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let mvp = projection_matrix * view_matrix * renderer.get_model_matrix();

        self.frame
            .draw(
                &renderer.model.vertex_buffer,
                &renderer.model.index_buffer,
                &self.program,
                &uniform! {
                    mvp: mvp,
                    color: renderer.color,
                    tex: &renderer.model.texture,
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
