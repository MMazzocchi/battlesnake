/*
 * S = Snake
 * H = Enemy Head
 * X = Our Head
 * W = Wall
 *
 * |---|
 * |SX |
 * |H  |
 * |WW |
 * |---|
 */

use crate::game::{
  direction::Direction,
  point::Point,
  space_type::SpaceType
};

use crate::snakes::strategy::rules::patterns::pattern::Pattern;

pub fn to_the_wall() -> Pattern {
  let mut pattern = Pattern::new(Direction::Right);

  pattern.set_space(Point { x: -1, y:  0 }, SpaceType::Snake);
  pattern.set_space(Point { x:  1, y:  0 }, SpaceType::Empty);
  pattern.set_space(Point { x: -1, y: -1 }, SpaceType::Head);
  pattern.set_space(Point { x:  0, y: -1 }, SpaceType::Empty);
  pattern.set_space(Point { x: -1, y: -2 }, SpaceType::Wall);
  pattern.set_space(Point { x:  0, y: -2 }, SpaceType::Wall);

  pattern
}
