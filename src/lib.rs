#[macro_use]
extern crate glium;

pub mod game;
pub mod render;
pub mod utils;

pub use crate::game::{run_game, Context, Game, GameConfig, KeyCode};
pub use crate::render::{Canvas, Model};
