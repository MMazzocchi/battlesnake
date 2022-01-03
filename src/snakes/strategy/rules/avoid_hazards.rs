use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake,
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct AvoidHazards {}

impl Rule for AvoidHazards {
  fn get_name(&self) -> &str { "Avoid Hazards" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let mut valid_options = Vec::<Direction>::new();

    for option in &options {
      let destination = you.head.apply(*option);
      if !game.is_hazard(&destination) {
        valid_options.push(*option);
      }
    }

    valid_options
  }
}
