use serde::Deserialize;
use crate::messages::game::Game;
use crate::messages::board::Board;

#[derive(Deserialize)]
pub struct End {
  pub game: Game,
  pub turn: i32,
  pub board: Board
}
