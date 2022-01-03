use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct NotInHazard {
  rule: Box<dyn Rule>,
  name: String
}

impl NotInHazard {
  pub fn new(rule: Box<dyn Rule>) -> Self {
    let name = format!("Not In Hazard: {}", &rule.get_name());
    NotInHazard {
      rule: rule,
      name: name
    }
  }
}

impl Rule for NotInHazard {
  fn get_name(&self) -> &str { &self.name }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    if !game.is_hazard(&you.head) {
      return self.rule.execute(game, you, options);
    }

    debug!("[{} {}] In a hazard. Skipping.", game.id, game.turn);
    return options;
  }
}
