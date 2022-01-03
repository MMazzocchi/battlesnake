use log::{
  info,
  error
};

use actix_web::{
  Responder,
  web::{
    Data,
    Json
  }
};

use crate::app_state::AppState;
use crate::messages::start::Start;

pub async fn start_handler(message: Json<Start>, data: Data::<AppState>) -> impl Responder {
  info!("Received start message");

  match data.game_manager.lock() {
    Ok(mut game_mgr) => {
      game_mgr.create_game(&message.game, &message.board);
    }

    _ => {
      error!("Game manager mutex was poisoned! This game will not be created.");
    }
  }

  ""
}
