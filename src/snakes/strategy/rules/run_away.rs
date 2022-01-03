/*
 * For each available option, this rule finds the distance it places the snake's head from all other
 * snakes' heads. It then returns only the options that increase that distance the most.
 */

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct RunAway {}

// Why not average? Because, in short, it doesn't matter. Everything gets divided by the same
// number, so let's skip the div.

fn total_distance_from_heads(direction: &Direction, game: &Game, you: &Battlesnake) -> i32 {
  let next_point = you.head.apply(*direction);
  let mut total: i32 = 0;

  for snake in &game.snakes {
    if snake.id != you.id {
      total += next_point.path_distance(&snake.head);
    }
  }

  total
}

impl Rule for RunAway {
  fn get_name(&self) -> &str { "Run Away" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let mut best_options = Vec::<Direction>::new();
    let mut highest_total: i32 = 0;

    for option in &options {
      let total_distance = total_distance_from_heads(option, game, you);

      if total_distance > highest_total {
        best_options.clear();
        best_options.push(*option);
        highest_total = total_distance;

      } else if total_distance == highest_total {
        best_options.push(*option);
      }
    }

    best_options
  }
}
