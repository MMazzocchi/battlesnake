use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  point::Point,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct FoodSafety {}

impl Rule for FoodSafety {
  fn get_name(&self) -> &str { "FoodSafety" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let furthest_distance = game.last_row + game.last_col;

    let mut safest_danger = -1;
    let mut safest_food: Option<Point> = None;

    for food in &game.food {
      // Base danger: how far away is it?
      let mut danger = you.head.path_distance(food) * 4;

      // Snakes close by increase danger
      for snake in &game.snakes {
        if snake.id != you.id {
          let distance = snake.head.path_distance(food);
          danger = danger + furthest_distance - distance;
        }
      }

      // Ignore food in hazards
      if game.is_hazard(food) {
        continue;
      }

      match safest_food {
        Some(safe_food) => {
          if safest_danger == danger {
            // Tie-breaker: distance
            let distance_to_food = you.head.path_distance(food);
            let distance_to_safe_food = you.head.path_distance(&safe_food);

            if distance_to_food < distance_to_safe_food {
              safest_food = Some(*food);
              safest_danger = danger;
            }
          } else if danger < safest_danger {
            safest_food = Some(*food);
            safest_danger = danger;
          }
        }
        None => {
          safest_danger = danger;
          safest_food = Some(*food);
        }
      }
    }

    match safest_food {
      Some(food) => {
        return you.head.best_direction_to(&food, &options);
      }

      _ => {
        debug!("[{} {}] Couldn't find any food. Skipping.", game.id, game.turn);
        return options;
      }
    }
  }
}
