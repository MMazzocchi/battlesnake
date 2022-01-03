use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct MaxSnakes {
  rule: Box<dyn Rule>,
  name: String,
  threshold: usize
}

impl MaxSnakes {
  pub fn new(rule: Box<dyn Rule>, threshold: usize) -> Self {
    let name = format!("Snakes <= {}: {}", threshold, &rule.get_name());
    MaxSnakes {
      rule: rule,
      name: name,
      threshold: threshold
    }
  }
}

impl Rule for MaxSnakes {
  fn get_name(&self) -> &str { &self.name }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    if &game.snakes.len() > &self.threshold {
      debug!("[{} {}] Snakes above threshold. Skipping.", game.id, game.turn);
      return options;
    }

    return self.rule.execute(game, you, options);
  }
}
