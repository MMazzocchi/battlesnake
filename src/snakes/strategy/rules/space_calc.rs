/*
 * For every direction, determine the size of the space the snake is moving into. If the space is
 * large enough to accomodate the current snake's length, it is returned along with all other
 * directions that also fulfill this requirement. If no options fulfill this requirement, then the
 * directions that move the snake into the largest space are returned.
 */

use log::debug;

use crate::game::{
  direction::Direction,
  game::Game,
  point::Point,
  space_type::SpaceType,
  battlesnake::Battlesnake
};

use crate::snakes::strategy::rules::rule::Rule;

struct SpaceData {
  pub space_available: i32,
  pub enough_space: bool,
  pub contains_tail: bool
}

#[derive(Clone)]
pub struct SpaceCalc {}

fn enough_space(direction: Direction, game: &Game, you: &Battlesnake) -> SpaceData {
  let mut data = SpaceData {
    space_available: 0,
    contains_tail: false,
    enough_space: false
  };

  let directions = vec![ Direction::Up, Direction::Down, Direction::Left, Direction::Right ];

  let mut points_to_check: Vec<Point> = vec![ you.head.apply(direction) ];
  let mut points_checked: Vec<Point> = vec![ you.head ]; 

  while !points_to_check.is_empty() {
    let point = points_to_check.pop().unwrap();
    points_checked.push(point);

    let space_type = game.get_space_type(&point);
    if space_type.is_navigable() {
      data.space_available += 1;

      if space_type == SpaceType::Tail {
        data.contains_tail = true;
      }


      if data.space_available >= you.length {
        data.enough_space = true;
        return data;
      }

      for next_direction in &directions {
        let next_point = point.apply(*next_direction);
        if !points_checked.contains(&next_point) {
          points_to_check.push(next_point);
        }
      }
    }
  }

  // Out of points!
  return data;
}

impl Rule for SpaceCalc {
  fn get_name(&self) -> &str { "Space Calc" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let mut enough_space_directions = Vec::<Direction>::new();

    let mut largest_space_directions = Vec::<Direction>::new();
    let mut largest_space: i32 = 0;

    let mut tail_directions = Vec::<Direction>::new();

    for option in options {
      let data: SpaceData = enough_space(option, game, you);
      debug!("[{} {}] {}: {} spaces, enough space: {}", game.id, game.turn,
        option.to_string(), data.space_available, data.enough_space);

      if data.enough_space {
        enough_space_directions.push(option);
      }

      if data.contains_tail {
        tail_directions.push(option);
      }

      if data.space_available > largest_space {
        largest_space_directions.clear();
        largest_space_directions.push(option);
        largest_space = data.space_available;

      } else if data.space_available == largest_space {
        largest_space_directions.push(option);
      }
    }

    if enough_space_directions.is_empty() {
      if tail_directions.is_empty() {
        debug!("[{} {}] Using the largest spaces", game.id, game.turn);
        return largest_space_directions;

      } else {
        debug!("[{} {}] Moving towards tail", game.id, game.turn);
        return tail_directions;
      }

    } else {
      debug!("[{} {}] Moving into large enough spaces", game.id, game.turn);
      return enough_space_directions;
    }
  }
}
