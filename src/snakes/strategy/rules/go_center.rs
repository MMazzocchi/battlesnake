use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake,
  point::Point
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct GoCenter {}

impl Rule for GoCenter {
  fn get_name(&self) -> &str { "Go Center" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    debug!("[{} {}] Center: {}, {}", game.id, game.turn, game.center.x, game.center.y);

    let furthest_distance: i32 = game.last_col + game.last_row + 1;
    let mut closest_distance = furthest_distance;
    let mut best_directions = Vec::<Direction>::new();

    for direction in options {
      let next: Point = you.head.apply(direction);
      let current_distance: i32 = next.path_distance(&game.center);

      if current_distance < closest_distance {
        best_directions.clear();
        best_directions.push(direction);
        closest_distance = current_distance;

      } else if current_distance == closest_distance {
        best_directions.push(direction);
      }
    }

    return best_directions;
  }
}
