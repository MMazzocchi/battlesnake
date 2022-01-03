use log::{
  warn,
  info,
  debug
};
use rand::seq::SliceRandom;

use crate::game::{
  direction::Direction,
  battlesnake::Battlesnake,
  game::Game
};

use crate::snakes::strategy::rules::rule::Rule;

pub struct Strategy {
  pub rules: Vec<Box<dyn Rule>>
}

impl Strategy {
  pub fn new() -> Self {
    Strategy {
      rules: Vec::<Box<dyn Rule>>::new()
    }
  }

  pub fn _set_rules(&mut self, rules: Vec::<Box<dyn Rule>>) {
    self.rules = rules;
  }

  pub fn add_rule(&mut self, rule: Box<dyn Rule>) {
    self.rules.push(rule);
  }

  pub fn execute(&self, game: &Game, you: &Battlesnake) -> Direction {
    let mut options = vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down];

    for rule in &self.rules {
      debug!("[{} {}] Applying rule {}", game.id, game.turn, rule.get_name());

      let mut filtered_options = rule.execute(game, you, options);

      match filtered_options.len() {
        0 => {
          warn!("[{} {}] Out of options!", game.id, game.turn);
          return Direction::Up;
        }
        1 => {
          return filtered_options.pop().unwrap();
        }
        _ => {
          options = filtered_options;
        }
      }
    }

    info!("[{} {}] Out of rules. Moving randomly from remaining options.", game.id, game.turn);
    return *options.choose(&mut rand::thread_rng()).unwrap_or(&Direction::Up);
  }
}
