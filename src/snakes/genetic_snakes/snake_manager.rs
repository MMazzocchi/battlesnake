use std::collections::HashMap;

use log::{
  info,
  warn,
  debug
};

use rand::Rng;

use crate::{
  messages::end::End,
  snakes::{
    snake_manager::SnakeManager,
    snake::Snake,
    genetic_snakes::generate_random::{
      generate_random_strategy,
      get_random_rule
    },
    strategy::{
      strategy::Strategy,
      rules::rule::Rule
    }
  }
};

struct SnakeData {
  snake: Snake,
  games_played: u32,
  total_score: u32
}

pub struct GeneticSnakeManager {
  snakes: HashMap<String, SnakeData>,
  games_played: u32
}

impl GeneticSnakeManager {
  pub fn new() -> Self {
    let mut manager = GeneticSnakeManager {
      snakes: HashMap::new(),
      games_played: 0
    };

    for num in 0..4 {
      let host = format!("battlesnake-genetic-{}.maxmazzocchi.com", num);

      manager.snakes.insert(host.to_owned(),
        SnakeData {
          snake: Snake {
            name: "Genetic Snake".to_owned(),
            host: host.to_owned(),
            color: "#000000".to_owned(),
            head: "default".to_owned(),
            tail: "default".to_owned(),
            strategy: generate_random_strategy()
          },
          games_played: 0,
          total_score: 0
        });
    }

    manager.log_strategies();

    return manager;
  }

  fn log_strategies(&self) {
    for (host, snake_data) in self.snakes.iter() {
      debug!("Snake {} strategy:", host);

      for rule in &snake_data.snake.strategy.rules {
        debug!("  - {}", rule.get_name());
      }
    }
  }
}

impl SnakeManager for GeneticSnakeManager {
  fn get_snake(&self, host: &str) -> Option<&Snake> {
    match self.snakes.get(host) {
      Some(snake_data) => { return Some(&snake_data.snake); }
      _ => { return None; }
    }
  }

  fn end_game(&mut self, end_msg: &End, host: &str) {
    self.games_played += 1;

    match self.snakes.get_mut(host) {
      Some(snake_data) => {
        snake_data.games_played = 1;
        snake_data.total_score = 3 - (end_msg.board.snakes.len() as u32);
      }
      None => { warn!("No snake was found for host {}. Not adjusting any snake's score.", host); }
    }

    if self.games_played >= 32 {
      debug!("Building next generation...");

      self.games_played = 0;

      // Create a map of host => score
      let hosts: Vec<String> = self.snakes.keys().cloned().collect();
      let mut scores = HashMap::new();

      for host in &hosts {
        let snake = self.snakes.get(host).unwrap();
        let score = (snake.total_score as f32) / (snake.games_played as f32);
        info!("{} score: {}", host, score);

        scores.insert(String::from(host), score);
      }

      let mut score_entries: Vec<(&String, &f32)> = scores.iter().collect();
      score_entries.sort_by(|entry1, entry2| (entry2.1.partial_cmp(&entry1.1)).unwrap());

      // Get top snakes
      let num_top_snakes = hosts.len() / 2;
      let mut top_snakes: Vec::<SnakeData> = score_entries.iter().map(|entry| self.snakes.remove(entry.0).unwrap()).collect();
      top_snakes.truncate(num_top_snakes);

      for host in hosts {

        // Cross breed - Construct a new rule set from the top rule sets
        let mut rules = Vec::<Box<dyn Rule>>::new();
        let max_len = top_snakes.iter().map(|snake_data| snake_data.snake.strategy.rules.len()).max().unwrap_or(0);

        for index in 0..max_len {
          let top_snakes_index = rand::thread_rng().gen_range(0..top_snakes.len());
          let rule_set = &mut top_snakes[top_snakes_index].snake.strategy.rules;

          if rule_set.len() > index {
            rules.push(rule_set.get(index).unwrap().clone());
          }
        }

        // Mutations
        for _mutation in 0..4 {
          match rand::thread_rng().gen_range(0..2) {

            // Insert rule
            0 => {
              let rule_index = rand::thread_rng().gen_range(0..rules.len());
              rules.insert(rule_index, get_random_rule());
            } 

            // Remove rule
            _ => {
              let rule_index = rand::thread_rng().gen_range(0..rules.len());
              rules.remove(rule_index);
            } 
          }
        }

        let mut strategy = Strategy::new();
        strategy.set_rules(rules);

        self.snakes.insert(host.to_owned(),
          SnakeData {
            snake: Snake {
              name: "Genetic Snake".to_owned(),
              host: host.to_owned(),
              color: "#000000".to_owned(),
              head: "default".to_owned(),
              tail: "default".to_owned(),
              strategy: strategy
            },
            games_played: 0,
            total_score: 0
          });
      }      

      self.log_strategies();
    }
  }
}
