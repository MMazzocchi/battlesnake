/*
 * This rule finds the snake with the closest head, then filters out any move that does not move
 * the snake farther from the closest head. This move is skipped if no moves fulfill that
 * requirement.
 */

use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake,
  point::Point
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct RunAwayFromCloseSnakes {
  name: String,
  divisor: i32
}

impl RunAwayFromCloseSnakes {
  pub fn new(divisor: i32) -> Self {
    RunAwayFromCloseSnakes {
      divisor: divisor,
      name: format!("Run Away From Close Snakes ({})", divisor)
    }
  }
}

impl Rule for RunAwayFromCloseSnakes {
  fn get_name(&self) -> &str { &self.name }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let mut closest_distance: i32 =
      ((game.last_col * game.last_col) + (game.last_row * game.last_row)) / self.divisor;

    let mut closest_head: &Point = &Point { x: 0, y: 0 };
    let mut found: bool = false;

    for snake in &game.snakes {
      if snake.id != you.id {
        let head_distance: i32 = you.head.distance_sq(&snake.head);

        if head_distance <= closest_distance {
          closest_distance = head_distance;
          closest_head = &snake.head;
          found = true;
        }
      }
    }

    if !found {
      debug!("[{} {}] No snakes were within the threshold. Skipping this rule.",
        game.id, game.turn);
      return options;

    } else {
      debug!("[{} {}] Closest head is distance sqrt({})", game.id, game.turn, closest_distance);
  
      let mut best_options = Vec::<Direction>::new();
  
      for option in &options {
        let next = &you.head.apply(*option);
        let new_distance = next.distance_sq(closest_head);
  
        if new_distance > closest_distance {
          best_options.push(*option);
        }
      }
  
      best_options
    }
  }
}
