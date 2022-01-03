use crate::{
  snakes::snake::Snake,
  messages::end::End
};

pub trait SnakeManager {
  fn get_snake(&self, host: &str) -> Option<&Snake>;

  fn end_game(&mut self, _end_msg: &End, _host: &str) {}
}
