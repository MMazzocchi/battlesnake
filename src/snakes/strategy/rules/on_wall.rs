use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct OnWall {
  rule: Box<dyn Rule>,
  name: String
}

impl OnWall {
  pub fn new(rule: Box<dyn Rule>) -> Self {
    let name = format!("On Wall: {}", &rule.get_name());
    OnWall {
      rule: rule,
      name: name
    }
  }
}

impl Rule for OnWall {
  fn get_name(&self) -> &str { &self.name }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    if (you.head.x == 0) || (you.head.y == 0) ||
       (you.head.x == game.last_col) ||
       (you.head.y == game.last_row) {

      return self.rule.execute(game, you, options);
    }

    debug!("[{} {}] Not on a wall. Skipping.", game.id, game.turn);
    return options;
  }
}
