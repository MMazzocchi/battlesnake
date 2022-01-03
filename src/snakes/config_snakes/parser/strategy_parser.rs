use serde::Deserialize;

use crate::snakes::{
  config_snakes::parser::rule_factory::build_rule,
  strategy::strategy::Strategy
};

#[derive(Deserialize)]
pub struct RuleStruct {
  pub name: String,

  #[serde(default)]
  pub skippable: bool,

  #[serde(default)]
  pub parameters: Vec<String>,

  #[serde(default)]
  pub health: Option<i32>,

  #[serde(default)]
  pub size_advantage: Option<i32>,

  #[serde(default)]
  pub on_wall: bool,

  #[serde(default)]
  pub in_hazard: bool,

  #[serde(default)]
  pub not_in_hazard: bool,

  #[serde(default)]
  pub hazard_timer: bool,

  #[serde(default)]
  pub min_snakes: Option<usize>,

  #[serde(default)]
  pub max_snakes: Option<usize>,
}

#[derive(Deserialize)]
pub struct StrategyStruct {
  rules: Vec<RuleStruct>
}

pub fn make_strategy(strategy_struct: StrategyStruct) -> Strategy {
  let mut strategy: Strategy = Strategy::new();

  for rule_struct in &strategy_struct.rules {
    match build_rule(rule_struct) {
      Some(rule) => { strategy.add_rule(rule); }
      None => {}
    }
  }

  return strategy;
}
