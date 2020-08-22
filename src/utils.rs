use glium::uniforms::{AsUniformValue, UniformValue};

use std::ops::Mul;

pub type Vec2 = (f32, f32);
pub type Color = (f32, f32, f32);

#[derive(Copy, Clone)]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Mat4 {
    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

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

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn collides_with(&self, other: &Rectangle) -> bool {
        self.x + self.w >= other.x
            && self.x <= other.x + other.w
            && self.y + self.h >= other.y
            && self.y <= other.y + other.h
    }
}
