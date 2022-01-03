use std::collections::HashMap;
use crate::game::{
  game::Game,
  battlesnake::Battlesnake,
  space_type::SpaceType,
  point::Point,
  direction::Direction
};

type Transform = fn(Point) -> Point;
static TRANSFORMS: [Transform; 8] = [
  |p| { p },
  |p| { Point { x: -p.x, y:  p.y } },
  |p| { Point { x:  p.x, y: -p.y } },
  |p| { Point { x: -p.x, y: -p.y } },
  |p| { Point { x:  p.y, y:  p.x } },
  |p| { Point { x: -p.y, y:  p.x } },
  |p| { Point { x:  p.y, y: -p.x } },
  |p| { Point { x: -p.y, y: -p.x } },
];

#[derive(Clone)]
pub struct Pattern {
  pattern: HashMap<Point, SpaceType>,
  direction: Direction
}

impl Pattern {
  pub fn new(direction: Direction) -> Self {
    Pattern {
      pattern: HashMap::new(),
      direction: direction
    }
  }

  pub fn set_space(&mut self, head_offset: Point, space_type: SpaceType) {
    self.pattern.insert(head_offset, space_type);
  }

  pub fn matches(&self, game: &Game, you: &Battlesnake) -> Option<Direction> {

    'transforms: for transform in TRANSFORMS.iter() {
      for (offset, space_type) in self.pattern.iter() {

        // Do all the points in this pattern (transformed) match?
        let point = &you.head.add(transform(*offset));
        if game.get_space_type(point) != *space_type {
          continue 'transforms;
        }
      }

      // All points matched! Apply the transform to the direction.
      return transform(self.direction.to_point()).to_direction();
    }

    // No transforms matched.
    return None;
  }
}
