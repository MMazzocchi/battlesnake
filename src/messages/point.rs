use serde::Deserialize;

#[derive(Deserialize)]
pub struct Point {
  pub x: i32,
  pub y: i32
}
