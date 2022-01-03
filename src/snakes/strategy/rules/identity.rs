/*
 * Identity just returns the options given unchanged.
 */

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct Identity {}

impl Rule for Identity {
  fn get_name(&self) -> &str { "Identity" }

  fn execute(&self, _game: &Game, _you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    options
  }
}
