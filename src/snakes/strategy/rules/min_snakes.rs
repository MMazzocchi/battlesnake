use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct MinSnakes {
  rule: Box<dyn Rule>,
  name: String,
  threshold: usize
}

impl MinSnakes {
  pub fn new(rule: Box<dyn Rule>, threshold: usize) -> Self {
    let name = format!("Snakes >= {}: {}", threshold, &rule.get_name());
    MinSnakes {
      rule: rule,
      name: name,
      threshold: threshold
    }
  }
}

impl Rule for MinSnakes {
  fn get_name(&self) -> &str { &self.name }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    if &game.snakes.len() < &self.threshold {
      debug!("[{} {}] Snakes below threshold. Skipping.", game.id, game.turn);
      return options;
    }

    return self.rule.execute(game, you, options);
  }
}
