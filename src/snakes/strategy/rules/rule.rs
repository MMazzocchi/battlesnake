use crate::game::{
  direction::Direction,
  game::Game,
  battlesnake::Battlesnake
};

pub trait Rule: CloneRule {
  fn get_name(&self) -> &str;

  fn execute(&self, game: &Game, you: &Battlesnake, options: Vec<Direction>) -> Vec<Direction>;
}

pub trait CloneRule {
  fn clone_rule<'a>(&self) -> Box<dyn Rule>;
}

impl<T> CloneRule for T
  where T: Rule + Clone + 'static {

  fn clone_rule(&self) -> Box<dyn Rule> {
    return Box::new(self.clone());
  }
}

impl Clone for Box<dyn Rule> {
  fn clone(&self) -> Self {
    return self.clone_rule();
  }
}
