/*
 * This rule filters out any move that will takes the snake into a non-navigable space (wall,
 * another snake, etc).
 */

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct DontDie {}

impl Rule for DontDie {
  fn get_name(&self) -> &str { "Don't Die" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let mut valid_options = Vec::<Direction>::new();

    for option in options {
      let destination = you.head.apply(option);
      let space_type = game.get_space_type(&destination);

      if space_type.is_navigable() {
        valid_options.push(option);
      }
    }

    valid_options
  }
}
