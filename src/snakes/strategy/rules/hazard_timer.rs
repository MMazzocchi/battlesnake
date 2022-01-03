/*
 * Applies a rule if we're within N turns of a hazard appearing. For example, if hazards appear
 * every 20 turns and X is 3, apply a rule on turns 17, 18, 19; 27, 28, 29; 37, 38, 39; etc.
 *
 * Note that this does NOT trigger on the turn the hazards appear.
 */

use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct HazardTimer {
  rule: Box<dyn Rule>,
  name: String,
  threshold: i32,
  hazard_interval: i32
}

impl HazardTimer {
  pub fn new(rule: Box<dyn Rule>) -> Self {

    // Yes, these should be config-driven. I don't know how to do that though.
    let threshold: i32 = 3;
    let hazard_interval: i32 = 25;

    let name = format!("Hazard Timer ({}/{}): {}", hazard_interval, threshold, &rule.get_name());
    HazardTimer {
      rule: rule,
      name: name,
      threshold: threshold,
      hazard_interval: hazard_interval
    }
  }
}

impl Rule for HazardTimer {
  fn get_name(&self) -> &str { &self.name }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let time_to_hazard: i32 = self.hazard_interval - (game.turn % self.hazard_interval);
    if (time_to_hazard > 0) && (time_to_hazard <= self.threshold) {
      return self.rule.execute(game, you, options);
    }

    debug!("[{} {}] Not within threshold. Skipping.", game.id, game.turn);
    return options;
  }
}
