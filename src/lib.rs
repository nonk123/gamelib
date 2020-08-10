#[macro_use]
extern crate glium;

mod game;
mod render;

pub use crate::game::{run_game, Context, Game, GameConfig};
pub use crate::render::Canvas;
