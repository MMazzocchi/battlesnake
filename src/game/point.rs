use crate::{
  messages::point::Point as PointMessage,
  game::direction::Direction
};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
  pub x: i32,
  pub y: i32
}

impl Point {
 pub fn from_message(message: &PointMessage) -> Self {
    Point {
      x: message.x,
      y: message.y
    }
  }

  pub fn add(&self, point: Point) -> Point {
    Point {
      x: self.x + point.x,
      y: self.y + point.y,
    }
  }


  pub fn apply(&self, direction: Direction) -> Point {
    return self.add(direction.to_point());
  }

  pub fn path_distance(&self, point: &Point) -> i32 {
    let dx: i32 = self.x - point.x;
    let dy: i32 = self.y - point.y;
    dx.abs() + dy.abs()
  }

  pub fn to_direction(&self) -> Option<Direction> {
    match(self.x, self.y) {
      ( 0,  1) => Some(Direction::Up),
      ( 0, -1) => Some(Direction::Down),
      ( 1,  0) => Some(Direction::Right),
      (-1,  0) => Some(Direction::Left),
      _ => None
    }
  }

  pub fn distance_sq(&self, point: &Point) -> i32 {
    let dx: i32 = self.x - point.x;
    let dy: i32 = self.y - point.y;
    (dx*dx) + (dy*dy)
  }

  pub fn best_direction_to(&self, point: &Point, options: &Vec::<Direction>) -> Vec::<Direction> {
    let mut closest_distance: Option<i32> = None;
    let mut best_directions = Vec::<Direction>::new();

    for direction in options {
      let next: Point = self.apply(*direction);
      let current_distance: i32 = next.path_distance(&point);

      match closest_distance {
        Some(distance) => {
          if current_distance < distance {
            best_directions.clear();
            closest_distance = Some(current_distance);
          }

          if current_distance <= distance {
            best_directions.push(*direction);
          }
        }
        None => {
          best_directions.push(*direction);
          closest_distance = Some(current_distance);
        }
      }
    }

    return best_directions;
  }
}
