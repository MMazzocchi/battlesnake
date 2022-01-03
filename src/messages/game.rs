use serde::Deserialize;

#[derive(Deserialize)]
pub struct Game {
  pub id: String,
  pub timeout: i32
}
