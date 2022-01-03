use std::collections::HashMap;
use log::info;
use crate::messages::{
  game::Game as GameMessage,
  board::Board as BoardMessage
};
use crate::game::game::Game;

pub struct GameManager {
  games: HashMap<String, Game>
}

impl GameManager {
  pub fn new() -> Self {
    GameManager {
      games: HashMap::new()
    }
  }

  pub fn create_game(&mut self, game_message: &GameMessage, board_message: &BoardMessage) -> &Game {
    info!("Creating game with ID: {}", game_message.id);

    self.games.insert(game_message.id.to_owned(),
      Game::new(game_message.id.to_owned(), board_message));
    self.get_game(&game_message.id).unwrap()
  }

  pub fn get_game(&mut self, game_id: &str) -> Option<&mut Game> {
    self.games.get_mut(game_id)
  }

  pub fn end_game(&mut self, game_id: &str) {
    info!("Ending game {}", game_id);

    self.games.remove(game_id);
  }
}
