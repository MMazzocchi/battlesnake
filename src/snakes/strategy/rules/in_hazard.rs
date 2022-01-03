use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct InHazard {
  rule: Box<dyn Rule>,
  name: String
}

impl InHazard {
  pub fn new(rule: Box<dyn Rule>) -> Self {
    let name = format!("In Hazard: {}", &rule.get_name());
    InHazard {
      rule: rule,
      name: name
    }
  }
}

impl Rule for InHazard {
  fn get_name(&self) -> &str { &self.name }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    if game.is_hazard(&you.head) {
      return self.rule.execute(game, you, options);
    }

    debug!("[{} {}] Not in a hazard. Skipping.", game.id, game.turn);
    return options;
  }
}
