use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake,
};

use crate::snakes::strategy::rules::rule::Rule;

#[derive(Clone)]
pub struct AvoidHazardWalls {}

impl Rule for AvoidHazardWalls {
  fn get_name(&self) -> &str { "Avoid Hazard Walls" }

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction> {
    let mut valid_options = Vec::<Direction>::new();
    let mut least_wall_count: i32 = 2;

    let directions = vec![ Direction::Up, Direction::Down, Direction::Left, Direction::Right ];

    for option in &options {
      let destination = you.head.apply(*option);
      let mut wall_count: i32 = 0;

      for direction in &directions {
        let next_destination = destination.apply(*direction);

        if game.is_hazard(&next_destination) {
          wall_count += 1;
        }
      }

      if wall_count < least_wall_count {
        valid_options.clear();
        valid_options.push(*option);
        least_wall_count = wall_count;

      } else if wall_count == least_wall_count {
        valid_options.push(*option);
      }
   }

    valid_options
  }
}
