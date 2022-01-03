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
use crate::messages::r#move::Move;
use crate::game::{
  direction::random_direction,
  battlesnake::Battlesnake,
};
use crate::handlers::get_snake::get_snake;

use json::object;

pub async fn move_handler(
  message: Json<Move>,
  data: Data::<AppState>,
  request: HttpRequest
) -> impl Responder {

  info!("Received move message");

  let mut next_move = random_direction();

  match data.snake_manager.lock() {
    Ok(snake_mgr) => {
      match get_snake(&snake_mgr, request) {
        Some(snake) => {

          match data.game_manager.lock() {
            Ok(mut game_mgr) => {

              let you = Battlesnake::from_message(&message.you);
            
              match game_mgr.get_game(&message.game.id) {
                Some(game) => {
                  game.update(&message.board, message.turn);
                  next_move = snake.make_move(&game, &you);
                }
              
                None => {
                  warn!("No game was found. Instantiating one.");
                  let game = game_mgr.create_game(&message.game, &message.board);
                  next_move = snake.make_move(&game, &you);
                }
              }
            }

            _ => {
              error!("Game manager mutex was poisoned! Moving randomly.");
            }
          }
        }
        None => {
          warn!("No snake was found.");
        }
      }
    }

    _ => {
      error!("Snake manager mutex was poisoned! Moving randomly.");
    }
  }

  let data = object!{
    "move" => next_move.to_string()
  };

  data.dump()
    .with_header("content-type", "application/json")
}
