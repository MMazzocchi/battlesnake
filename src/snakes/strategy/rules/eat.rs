/*
 * If the snake's health is below a threshold, find the closest food and filter out any moves that
 * do not move the snake closer to that food. This rule is skipped if the snake's health is above
 * the threshold, or if there are no moves that take it closer.
 */

use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  point::Point,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct Eat {}

impl Rule for Eat {
  fn get_name(&self) -> &str { "Eat" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let furthest_distance: i32 = game.last_col + game.last_row + 1;

    // Find the closest food
    let mut closest_food: Option<&Point> = None;
    let mut food_distance: i32 = furthest_distance;

    for food in &game.food {
      let current_distance: i32 = you.head.path_distance(&food);

      if current_distance < food_distance {
        food_distance = current_distance;
        closest_food.replace(food);
      }
    }

    match closest_food {
      Some(food) => {

        // Which moves take us closest?
        let mut closest_distance = furthest_distance;
        let mut best_directions = Vec::<Direction>::new();

        for direction in options {
          let next: Point = you.head.apply(direction);
          let current_distance: i32 = next.path_distance(&food);

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

      _ => {
        debug!("[{} {}] Couldn't find any food. Skipping.", game.id, game.turn);
        return options;
      }
    }
  }
}
