use std::collections::HashMap;
use std::fs;

use log::{
  info,
  warn,
  error,
};

use serde::Deserialize;

use crate::snakes::{
  snake_manager::SnakeManager,
  snake::Snake,
  config_snakes::{
    snake_loader::load_snake,
    parser::strategy_parser::StrategyStruct
  }
};

#[derive(Deserialize)]
struct SnakeStruct {
  pub name: String,
  pub host: String,
  pub color: String,
  pub head: String,
  pub tail: String,
  pub strategy: StrategyStruct
}

pub struct ConfigSnakeManager {
  snakes: HashMap<String, Snake>
}

impl ConfigSnakeManager {
  pub fn new() -> Self {
    let mut manager = ConfigSnakeManager {
      snakes: HashMap::new()
    };

    match fs::read_dir("config/snakes") {
      Ok(entries) => {
        for entry in entries {
          match load_snake(entry) {
            Ok(snake) => {
              let name = &snake.name.to_owned();
              manager.snakes.insert(snake.host.to_owned(), snake);
              info!("Registered snake: {}", name);
            }
            Err(err) => {
              warn!("Failed to load snake configuration: {}", err);
            }
          }
        }
      }
      Err(err) => {
        error!("Failed to read snakes directory: {}", err);
      }
    }

    return manager;
  }
}

impl SnakeManager for ConfigSnakeManager {
  fn get_snake(&self, host: &str) -> Option<&Snake> {
    self.snakes.get(host)
  }
}
