/*
 * This rule filters out any moves that could potentially cause a losing head-to-head collision. If
 * the potential head-to-head is winning, then the move is not filtered. If there are no moves that
 * avoid the losing collision, this rule is skipped.
 */

use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake,
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct HeadToHead {}

fn safe_move(direction: Direction, game: &Game, you: &Battlesnake) -> bool {
  let next_point = you.head.apply(direction);

  for snake in &game.snakes {
    if snake.id != you.id {
      let distance = next_point.path_distance(&snake.head);
  
      if distance == 1 {
        if snake.length < you.length {
          return true;
        } else {
          return false;
        }
      }
    }
  }

  return true;
}

impl Rule for HeadToHead {
  fn get_name(&self) -> &str { "Head To Head" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let mut valid_options = Vec::<Direction>::new();

    for option in &options {
      let safe_move = safe_move(*option, game, you);

      if safe_move {
        valid_options.push(*option);
      }
    }

    return valid_options;
  }
}
