use serde::Deserialize;
use crate::messages::{
  point::Point,
  battlesnake::Battlesnake
};

#[derive(Deserialize)]
pub struct Board {
  pub height: i32,
  pub width: i32,
  pub food: Vec<Point>,
  pub hazards: Vec<Point>,
  pub snakes: Vec<Battlesnake>
}
