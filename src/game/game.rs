use std::collections::HashMap;

use crate::messages::board::Board as BoardMessage;
use crate::game::{
  space_type::SpaceType,
  point::Point,
  battlesnake::Battlesnake
};

pub struct Game {
  pub id: String,
  pub turn: i32,
  pub last_col: i32,
  pub last_row: i32,
  pub positions: HashMap<Point, SpaceType>,
  pub food: Vec<Point>,
  pub hazards: Vec<Point>,
  pub snakes: Vec<Battlesnake>,
  pub center: Point,
  north_center: Point,
  east_center: Point,
  south_center: Point,
  west_center: Point
}

impl Game {
  pub fn new(id: String, board_message: &BoardMessage) -> Self {
    let mut game = Game {
      id: id,
      turn: 0,
      last_col: board_message.width - 1,
      last_row: board_message.height - 1,
      positions: HashMap::new(),
      food: Vec::<Point>::new(),
      hazards: Vec::<Point>::new(),
      snakes: Vec::<Battlesnake>::new(),
      center: Point {
        x: board_message.width / 2,
        y: board_message.height / 2
      },
      north_center: Point {
        x: board_message.width / 2,
        y: board_message.height - 1
      },
      east_center: Point {
        x: board_message.width - 1,
        y: board_message.height / 2
      },
      south_center: Point {
        x: board_message.width / 2,
        y: 0
      },
      west_center: Point {
        x: 0,
        y: board_message.height / 2
      }
    };

    game.update(board_message, 0);
    game
  }

  pub fn update(&mut self, board_message: &BoardMessage, turn: i32) {
    self.turn = turn;

    self.positions.clear();
    self.food.clear();
    self.snakes.clear();

    for food in &board_message.food {
      self.positions.insert(Point::from_message(&food), SpaceType::Food);
      self.food.push(Point::from_message(&food));
    }

    for hazard in &board_message.hazards {
      self.hazards.push(Point::from_message(&hazard));
      let hazard_pt: Point = Point::from_message(&hazard);

      if hazard_pt == self.north_center {
        self.north_center.y -= 1;
      }

      if hazard_pt == self.east_center {
        self.east_center.x -= 1;
      }

      if hazard_pt == self.south_center {
        self.south_center.y += 1;
      }

      if hazard_pt == self.west_center {
        self.west_center.x += 1;
      }

      self.center = Point {
        x: (self.east_center.x + self.west_center.x) / 2,
        y: (self.north_center.y + self.south_center.y) / 2
      };

      self.north_center.x = self.center.x;
      self.east_center.y = self.center.y;
      self.south_center.x = self.center.x;
      self.west_center.y = self.center.y;
    }

    for snake in &board_message.snakes {
      self.snakes.push(Battlesnake::from_message(&snake));

      // If the game has JUST started (or the snake just ate), the snake is "stacked"; the tail is
      // not going to disappear.
      let mut stacked_snake = false;

      for body_position in &snake.body {
        let new_point = Point::from_message(&body_position);
        match self.get_space_type(&new_point) {
          SpaceType::Snake => { stacked_snake = true; }
          _ => {}
        }

        self.positions.insert(Point::from_message(&body_position), SpaceType::Snake);
      }

      if !stacked_snake {
        let tail = &snake.body[snake.body.len() - 1];
        self.positions.insert(Point::from_message(&tail), SpaceType::Tail);
      }

      self.positions.insert(Point::from_message(&snake.body[0]), SpaceType::Head);
    }
  }

  pub fn get_space_type(&self, point: &Point) -> SpaceType {
    if (point.x < 0) || (point.x > self.last_col) ||
       (point.y < 0) || (point.y > self.last_row) {
      return SpaceType::Wall;
    }

    match self.positions.get(point) {
      Some(space_type) => { *space_type }
      _ => { SpaceType::Empty }
    }
  }

  pub fn is_hazard(&self, point: &Point) -> bool {
    return self.hazards.contains(point);
  }
}
