#[macro_use]
extern crate glium;

mod game;
mod render;

pub use crate::game::{run_game, Context, Game, GameConfig, KeyCode};
pub use crate::render::{Canvas, Model};
