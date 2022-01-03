use crate::game::point::Point;
use crate::messages::battlesnake::Battlesnake as BattlesnakeMessage;

pub struct Battlesnake {
  pub id: String,
  pub head: Point,
  pub tail: Point,
  pub length: i32,
  pub health: i32
}

impl Battlesnake {
 pub fn from_message(message: &BattlesnakeMessage) -> Self {
    Battlesnake {
      id: message.id.to_owned(),
      head: Point::from_message(&message.head),
      tail: Point::from_message(&message.body[message.body.len() - 1]),
      length: message.length,
      health: message.health
    }
  }
}
