use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct HealthThreshold {
  rule: Box<dyn Rule>,
  name: String,
  threshold: i32
}

impl HealthThreshold {
  pub fn new(rule: Box<dyn Rule>, threshold: i32) -> Self {
    let name = format!("Health < {}: {}", threshold, &rule.get_name());
    HealthThreshold {
      rule: rule,
      name: name,
      threshold: threshold
    }
  }
}

impl Rule for HealthThreshold {
  fn get_name(&self) -> &str { &self.name }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    if &you.health >= &self.threshold {
      debug!("[{} {}] Health above threshold. Skipping.", game.id, game.turn);
      return options;
    }

    return self.rule.execute(game, you, options);
  }
}
