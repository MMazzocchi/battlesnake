use crate::game::{
  direction::{
    Direction,
    all_directions
  },
  game::Game,
  point::Point,
  battlesnake::Battlesnake,
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct EscapeHazards {}

fn closest_non_hazard(game: &Game, start: Point) -> Option<Point> {
  let max_iterations: i32 = 10;
  let mut iteration: i32 = 0;
  let directions: Vec<Direction> = all_directions();

  // Simple BFS
  let mut points_to_check: Vec<Point> = vec![ start ];
  let mut points_checked: Vec<Point> = Vec::<Point>::new();

  while !points_to_check.is_empty() {
    iteration += 1;

    let point = points_to_check.remove(0);
    points_checked.push(point);

    let space_type = game.get_space_type(&point);
    if space_type.is_navigable() {
      if !game.is_hazard(&point) {
        return Some(point);

      } else {
        for next_direction in &directions {
          let next_point = point.apply(*next_direction);
          if !points_checked.contains(&next_point) {
            points_to_check.push(next_point);
          }
        }
      }
    }

    if iteration >= max_iterations {
      return None;
    }
  }

  return None;
}

impl Rule for EscapeHazards {
  fn get_name(&self) -> &str { "Escape Hazards" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let mut best_options = Vec::<Direction>::new();
    let mut furthest_distance: i32 = game.last_col + game.last_row;

    for option in &options {
      let start: Point = you.head.apply(*option);
      
      match closest_non_hazard(game, start) {
        Some(non_hazard) => {
          let distance = you.head.path_distance(&non_hazard);
          if distance <= furthest_distance {
            if distance < furthest_distance {
              furthest_distance = distance;
              best_options.clear();
            }

            best_options.push(*option);
          }
        }

        _ => {}
      }
    }

    best_options
  }
}
