use actix_web::{
  App,
  HttpServer,
  web
};
use actix_files::Files;
use actix_web::middleware::Logger;

use log4rs;

use log::info;
use std::sync::Mutex;

mod handlers;
mod app_state;
mod messages;
mod game;
mod snakes;

use handlers::{
  root_handler::root_handler,
  start_handler::start_handler,
  move_handler::move_handler,
  end_handler::end_handler
};

use game::game_manager::GameManager;
// use snakes::genetic_snakes::snake_manager::GeneticSnakeManager;
use snakes::config_snakes::snake_manager::ConfigSnakeManager;
use app_state::AppState;

const PORT: &str = "10000";
const HOST: &str = "0.0.0.0";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

  let mut url: String = "".to_owned();
  url.push_str(HOST);
  url.push_str(":");
  url.push_str(PORT);

  let server_future = HttpServer::new(move || {

    App::new()
      .app_data(web::Data::new(AppState {
        game_manager: Mutex::new(GameManager::new()),
        snake_manager: Mutex::new(Box::new(ConfigSnakeManager::new()))
      }))
      .wrap(Logger::default())
      .route("/", web::get().to(root_handler))
      .route("/start", web::post().to(start_handler))
      .route("/move", web::post().to(move_handler))
      .route("/end", web::post().to(end_handler))
      .service(Files::new("/", "./static"))
  })
  .workers(1)
  .bind(url)?
  .run();

  info!("Listening on {}", PORT);

  return server_future.await;
}
