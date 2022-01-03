use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake,
  point::Point
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct ChaseOtherSnakes {}

impl Rule for ChaseOtherSnakes {
  fn get_name(&self) -> &str { "Chase Other Snakes" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let mut closest_distance: i32 = -1;
    let mut closest_head: &Point = &Point { x: 0, y: 0 };

    // Find the closest snake smaller than us
    for snake in &game.snakes {
      if snake.id != you.id {
        let distance: i32 = you.head.distance_sq(&snake.head);

        if ((closest_distance == -1) || (distance < closest_distance)) &&
           (you.length > snake.length) {

          closest_distance = distance;
          closest_head = &snake.head;
        }
      }
    }

    if closest_distance == -1 {
      debug!("[{} {}] All snakes are larger. Skipping this rule.", game.id, game.turn);
      return options; 
    }

    // Pick the option that moves us the closest to that head (if any)
    let mut new_closest_distance: i32 = closest_distance;
    let mut best_options = Vec::<Direction>::new();

    for option in &options {
      let new_head: Point = you.head.apply(*option);
      let new_distance: i32 = new_head.distance_sq(closest_head);

      if new_distance <= new_closest_distance {
        if new_distance < new_closest_distance {
          new_closest_distance = new_distance;
          best_options.clear();
        }

        best_options.push(*option);
      }
    }

    if best_options.is_empty() {
      debug!("No options moved us closer. Skipping this rule.");
      return options;
    }

    return best_options;
  }
}
