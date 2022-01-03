use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct SizeAdvantage {
  rule: Box<dyn Rule>,
  name: String,
  threshold: i32
}

impl SizeAdvantage {
  pub fn new(rule: Box<dyn Rule>, threshold: i32) -> Self {
    let name = format!("Size Advantage {}: {}", threshold, &rule.get_name());
    SizeAdvantage {
      rule: rule,
      name: name,
      threshold: threshold
    }
  }
}

impl Rule for SizeAdvantage {
  fn get_name(&self) -> &str { &self.name }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {

    // Find any snakes within the threshold of our size
    let mut found: bool = false;
    for snake in &game.snakes {
      if snake.id != you.id {
        if (you.length - snake.length) < self.threshold {
          found = true;
          break;
        }
      }
    }

    if !found {
      debug!("[{} {}] No snakes within threshold. Skipping.", game.id, game.turn);
      return options;
    }

    return self.rule.execute(game, you, options);
  }
}
