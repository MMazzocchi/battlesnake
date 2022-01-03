use serde::Deserialize;
use crate::messages::game::Game;
use crate::messages::board::Board;
use crate::messages::battlesnake::Battlesnake;

#[derive(Deserialize)]
pub struct Start {
  pub game: Game,
  pub turn: i32,
  pub board: Board,
  pub you: Battlesnake
}
