use std::{
  fs,
  fs::DirEntry
};

use log::{
  debug,
};

use serde::Deserialize;

use serde_yaml;

use crate::snakes::{
  snake::Snake,
  config_snakes::parser::strategy_parser::{
    make_strategy,
    StrategyStruct
  }
};

#[derive(Deserialize)]
struct SnakeStruct {
  pub name: String,
  pub host: String,
  pub color: String,
  pub head: String,
  pub tail: String,
  pub author: String,
  pub strategy: StrategyStruct
}

use std::fmt::{
  Display,
  Formatter,
  Error
};
use std::io::Error as IoError;

pub struct SnakeLoadError {
  pub err: String
}

impl Display for SnakeLoadError {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}", &self.err)
  }
}

pub fn load_snake(entry: Result<DirEntry, IoError>) -> Result<Snake, SnakeLoadError> {
  let path = entry
    .map_err(|_| SnakeLoadError { err: "Failed to open directory entry".to_owned() })
    .map(|entry_value| entry_value.path() )?;

  let path_str = path.to_str()
    .ok_or(SnakeLoadError { err: "Failed to stringify path".to_owned() })?;

  let contents = fs::read_to_string(path_str)
    .map_err(|_| SnakeLoadError { err: "Could not read file".to_owned() })?;

  debug!("YAML:\n{}", contents);

  let snake_struct: SnakeStruct = serde_yaml::from_str(&contents)
    .map_err(|_| SnakeLoadError { err: "Could not deserialize YAML".to_owned() })?;

  let strategy = make_strategy(snake_struct.strategy);

  return Ok(
    Snake {
      name: snake_struct.name,
      host: snake_struct.host,
      color: snake_struct.color,
      head: snake_struct.head,
      tail: snake_struct.tail,
      author: snake_struct.author,
      strategy: strategy
    });
}
