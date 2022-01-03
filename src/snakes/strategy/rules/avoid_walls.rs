/* Avoid walls, if possible. This rule selects the move that places the snake against the least
 * number of walls.
 */

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct AvoidWalls {}

impl Rule for AvoidWalls {
  fn get_name(&self) -> &str { "Avoid Walls" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let mut valid_options = Vec::<Direction>::new();
    let mut least_wall_count: i32 = 4;

    for option in &options {
      let mut wall_count: i32 = 0;
      let destination = you.head.apply(*option);

      if (destination.x == 0) || (destination.x == game.last_col) {
        wall_count += 1;
      }

      if (destination.y == 0) || (destination.y == game.last_row) {
        wall_count += 1;
      }

      if wall_count < least_wall_count {
        valid_options.clear();
        valid_options.push(*option);
        least_wall_count = wall_count;
      } else if wall_count == least_wall_count {
        valid_options.push(*option);
      }
    }

    valid_options
  }
}
