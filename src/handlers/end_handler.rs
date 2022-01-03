use log::{
  info,
  warn,
  error
};

use actix_web::{
  Responder,
  web::{
    Data,
    Json,
    HttpRequest
  }
};

use crate::app_state::AppState;
use crate::messages::end::End;

pub async fn end_handler(message: Json<End>, data: Data::<AppState>, request: HttpRequest)
  -> impl Responder {

  info!("Received end message");

  let host_header = request.headers().get("host");
  match host_header {
    Some(host_header_val) => {
      let host_name = host_header_val.to_str().unwrap();

      match data.snake_manager.lock() {
        Ok(mut snake_mgr) => {
          snake_mgr.end_game(&message, &host_name);
        }

        _ => {
          error!("Snake manager mutex was poisoned! Not counting this game's score.");
        }
      }
    }
    None => {
      warn!("No host header was present! Not counting this game's score.")
    }
  }

  let mut game_mgr = data.game_manager.lock().unwrap();
  game_mgr.end_game(&message.game.id);

  ""
}
