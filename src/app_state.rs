use crate::game::game_manager::GameManager;
use crate::snakes::snake_manager::SnakeManager;
use std::sync::Mutex;

pub struct AppState {
  pub game_manager: Mutex<GameManager>,
  pub snake_manager: Mutex<Box::<dyn SnakeManager>>
}
