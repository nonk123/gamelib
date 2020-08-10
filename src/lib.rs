#[macro_use]
extern crate glium;

mod game;
mod render;

pub use crate::game::{run_game, Game};
pub use crate::render::Canvas;
