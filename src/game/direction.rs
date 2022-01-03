use rand::seq::SliceRandom;

use crate::game::point::Point;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right
}

impl Direction {
  pub fn to_string(&self) -> &str {
    match self {
      Direction::Up => "up",
      Direction::Down => "down",
      Direction::Left => "left",
      Direction::Right => "right"
    }
  }

  pub fn to_point(&self) -> Point {
    match self {
      Direction::Up =>    { Point { x:  0, y:  1 } },
      Direction::Down =>  { Point { x:  0, y: -1 } },
      Direction::Left =>  { Point { x: -1, y:  0 } },
      Direction::Right => { Point { x:  1, y:  0 } }
    }
  }
}

pub fn all_directions() -> Vec<Direction> {
  return vec![ Direction::Up, Direction::Down, Direction::Left, Direction::Right ];
}

pub fn random_direction() -> Direction {
  return *all_directions().choose(&mut rand::thread_rng()).unwrap_or(&Direction::Up);
}
