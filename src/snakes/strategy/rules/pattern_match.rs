use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::{
  rule::Rule,
  patterns::pattern::Pattern
};

#[derive(Clone)]
pub struct PatternMatch {
  pub pattern: Pattern,
  pub name: String
}

impl Rule for PatternMatch {
  fn get_name(&self) -> &str { &self.name }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let result: Option<Direction> = self.pattern.matches(game, you);

    match result {
      Some(direction) => {
        debug!("[{} {}] Pattern match!", game.id, game.turn);
        if options.contains(&direction) {
          return vec![ direction ];
        }
      }
      _ => {}
    }

    return Vec::<Direction>::new();
  }
}
