use serde::Deserialize;
use crate::messages::point::Point;

#[derive(Deserialize)]
pub struct Battlesnake {
  pub id: String,
  pub head: Point,
  pub body: Vec<Point>,
  pub length: i32,
  pub health: i32
}
