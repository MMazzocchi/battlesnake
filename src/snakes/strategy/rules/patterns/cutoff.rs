/*
 * Pattern match the diagramed pattern, and cutoff the opponent.
 *
 * S = Snake
 * H = Enemy Head
 * X = Our Head
 * W = Wall
 *
 * |----|
 * |SSX |
 * |H   |
 * |WWW |
 * |----|
 */

use crate::game::{
  direction::Direction,
  point::Point,
  space_type::SpaceType
};

use crate::snakes::strategy::rules::patterns::pattern::Pattern;

pub fn cutoff() -> Pattern {
  let mut pattern = Pattern::new(Direction::Down);

  pattern.set_space(Point { x: -2, y:  0 }, SpaceType::Snake);
  pattern.set_space(Point { x: -1, y:  0 }, SpaceType::Snake);
  pattern.set_space(Point { x: -2, y: -1 }, SpaceType::Head);
  pattern.set_space(Point { x: -1, y: -1 }, SpaceType::Empty);
  pattern.set_space(Point { x:  0, y: -1 }, SpaceType::Empty);
  pattern.set_space(Point { x:  1, y: -1 }, SpaceType::Empty);
  pattern.set_space(Point { x: -2, y: -2 }, SpaceType::Wall);
  pattern.set_space(Point { x: -1, y: -2 }, SpaceType::Wall);
  pattern.set_space(Point { x:  0, y: -2 }, SpaceType::Wall);

  pattern
}
