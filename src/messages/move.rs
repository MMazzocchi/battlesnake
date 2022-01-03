use serde::Deserialize;
use crate::messages::{
  game::Game,
  board::Board,
  battlesnake::Battlesnake
};

#[derive(Deserialize)]
pub struct Move {
  pub game: Game,
  pub turn: i32,
  pub board: Board,
  pub you: Battlesnake
}
