use log::warn;

use actix_web::web::HttpRequest;
use crate::snakes::{
  snake::Snake,
  snake_manager::SnakeManager
};

pub fn get_snake(snake_mgr: &Box<dyn SnakeManager>, request: HttpRequest) -> Option<&Snake> {
  let host_header = request.headers().get("host");
  match host_header {
    Some(host_header_val) => {
      let host_name = host_header_val.to_str().unwrap();
      let snake_opt = snake_mgr.get_snake(host_name);

      match snake_opt {
        Some(snake) => {
          return Some(snake);
        }
        None => {
          warn!("No snake was found for host {}", host_name);
        }
      }
    }
    None => {
      warn!("No host header was present! Cannot select a snake.")
    }
  }

  return None;
}
