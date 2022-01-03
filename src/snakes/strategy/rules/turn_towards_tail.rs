use crate::game::{
  direction::Direction,
  game::Game,
  point::Point,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct TurnTowardsTail {}

impl Rule for TurnTowardsTail {
  fn get_name(&self) -> &str { "Turn Towards Tail" }

  fn execute(&self, _game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let mut closest_distance: i32 = you.head.path_distance(&you.tail);

    // Which moves take us closest?
    let mut best_directions = Vec::<Direction>::new();

    for direction in options {
      let next: Point = you.head.apply(direction);
      let current_distance: i32 = next.path_distance(&you.tail);

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
