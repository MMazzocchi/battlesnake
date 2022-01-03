use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct Skippable {
  rule: Box<dyn Rule>,
  name: String,
}

impl Skippable {
  pub fn new(rule: Box<dyn Rule>) -> Self {
    let name = format!("Skippable: {}", &rule.get_name());
    Skippable {
      rule: rule,
      name: name 
    }
  }
}

impl Rule for Skippable {
  fn get_name(&self) -> &str { &self.name }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let old_options = options.to_vec();
    let filtered_options = self.rule.execute(game, you, options);

    match filtered_options.len() {
      0 => {
        debug!("[{} {}] No options returned from {}. Skipping.", game.id, game.turn,
          &self.rule.get_name());
        return old_options;
      }
      _ => {
        return filtered_options;
      }
    }
  }
}
