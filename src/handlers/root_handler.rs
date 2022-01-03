use log::{
  warn,
  error
};
use actix_web::{
  Responder,
  web::{
    Data,
    HttpRequest
  }
};
use json::object;

use crate::app_state::AppState;
use crate::handlers::get_snake::get_snake;

pub async fn root_handler(data: Data::<AppState>, request: HttpRequest) -> impl Responder {

  let version = env!("CARGO_PKG_VERSION");
  let mut color = "#000000".to_owned();
  let mut head = "default".to_owned();
  let mut tail = "default".to_owned();

  match data.snake_manager.lock() {
    Ok(snake_mgr) => {
      let snake_opt = get_snake(&snake_mgr, request);
    
      match snake_opt {
        Some(snake) => {
          color = snake.get_color().to_owned();
          head = snake.get_head().to_owned();
          tail = snake.get_tail().to_owned();
        }
        None => {
          warn!("No snake was found!");
        }
      }
    }

    _ => {
      error!("Snake manager mutex was poisoned! This snake's data will be standard.");
    }

  }

  let data = object!{
    "apiversion" => "1",
    "author" => "Max Mazzocchi <maxwell.mazzocchi@gmail.com>",
    "color" => color,
    "head" => head,
    "tail" => tail,
    "version" => version
  };

  data.dump()
    .with_header("content-type", "application/json")
}
