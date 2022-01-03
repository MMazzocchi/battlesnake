/*
 * Pattern match the diagramed pattern, and avoid being cornered.
 *
 * S = Snake
 * H = Enemy Head
 * X = Our Head
 * W = Wall
 *
 * |-----|
 * |    H|
 * |     |
 * |W X  |
 * |     |
 * |  W  |
 * |-----|
 */

use crate::game::{
  direction::Direction,
  point::Point,
  space_type::SpaceType,
};

use crate::snakes::strategy::rules::patterns::pattern::Pattern;

pub fn cornered() -> Pattern {
  let mut pattern = Pattern::new(Direction::Up);

  pattern.set_space(Point { x: -2, y:  0 }, SpaceType::Wall);
  pattern.set_space(Point { x:  0, y: -2 }, SpaceType::Wall);
  pattern.set_space(Point { x:  2, y:  2 }, SpaceType::Head);
  pattern.set_space(Point { x:  0, y:  1 }, SpaceType::Empty);

  pattern
}
