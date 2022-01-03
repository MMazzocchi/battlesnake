use crate::game::{
  direction::Direction,
  battlesnake::Battlesnake,
  game::Game
};

use crate::snakes::strategy::strategy::Strategy;

pub struct Snake {
  pub name: String,
  pub host: String,
  pub strategy: Strategy,
  pub color: String,
  pub head: String,
  pub tail: String,
  pub author: String
}

impl Snake {
  pub fn get_color(&self) -> &String {
    return &self.color;
  }

  pub fn get_head(&self) -> &String {
    return &self.head;
  }

  pub fn get_tail(&self) -> &String {
    return &self.tail;
  }

  pub fn get_author(&self) -> &String {
    return &self.author;
  }

  pub fn make_move(&self, game: &Game, you: &Battlesnake) -> Direction {
    return self.strategy.execute(&game, &you);
  }
}
