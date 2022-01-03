use rand::Rng;

use crate::snakes::strategy::{
  strategy::Strategy,
  rules::{
    rule::Rule,
    patterns::cornered::cornered,
    dont_die::DontDie,
    eat::Eat,
    space_calc::SpaceCalc,
    head_to_head::HeadToHead,
    run_away_from_close_snakes::RunAwayFromCloseSnakes,
    avoid_walls::AvoidWalls,
    pattern_match::PatternMatch,
    skippable::Skippable,
    chase_other_snakes::ChaseOtherSnakes,
    health_threshold::HealthThreshold,
    size_advantage::SizeAdvantage,
    food_safety::FoodSafety,
    on_wall::OnWall,
    run_away_path_dist::RunAwayFromCloseSnakesPathDist,
    avoid_hazards::AvoidHazards,
    avoid_hazard_walls::AvoidHazardWalls,
    turn_towards_tail::TurnTowardsTail,
    escape_hazards::EscapeHazards,
    in_hazard::InHazard,
    go_center::GoCenter,
    min_snakes::MinSnakes,
    max_snakes::MaxSnakes,
  }
};

fn get_random_base_rule() -> Box<dyn Rule> {
  let rule_index = rand::thread_rng().gen_range(0..15);
  match rule_index {
    1 => { return Box::new(Eat {}); }
    2 => { return Box::new(SpaceCalc {}); }
    3 => { return Box::new(HeadToHead {}); }
    4 => { return Box::new(AvoidWalls {}); }
    5 => { return Box::new(AvoidHazards {}); }
    6 => { return Box::new(FoodSafety {}); }
    7 => { return Box::new(ChaseOtherSnakes {}); }
    8 => { return Box::new(AvoidHazardWalls {}); }
    9 => { return Box::new(TurnTowardsTail {}); }
    10 => { return Box::new(EscapeHazards {}); }
    11 => { return Box::new(GoCenter {}); }
    12 => { return Box::new(PatternMatch {
      pattern: cornered(),
      name: "Cornered".to_string()
    }); }

    13 => {return Box::new(RunAwayFromCloseSnakes::new(3)); }
    14 => { return Box::new(RunAwayFromCloseSnakesPathDist::new(3)); }
    _ => { return Box::new(DontDie {}); }
  }
}

fn apply_random_modifiers(mut rule: Box<dyn Rule>) -> Box<dyn Rule> {
  let probability: f64 = 0.1;

  match rand::thread_rng().gen_bool(probability) {
    true => { rule = Box::new(OnWall::new(rule)); }
    _ => {}
  }

  match rand::thread_rng().gen_bool(probability) {
    true => { rule = Box::new(InHazard::new(rule)); }
    _ => {}
  }

  match rand::thread_rng().gen_bool(probability) {
    true => { rule = Box::new(Skippable::new(rule)); }
    _ => {}
  }

  match rand::thread_rng().gen_bool(probability) {
    true => { rule = Box::new(HealthThreshold::new(rule, rand::thread_rng().gen_range(0..101))); }
    _ => {}
  }

  match rand::thread_rng().gen_bool(probability) {
    true => { rule = Box::new(MaxSnakes::new(rule, rand::thread_rng().gen_range(2..4))); }
    _ => {}
  }

  match rand::thread_rng().gen_bool(probability) {
    true => { rule = Box::new(MinSnakes::new(rule, rand::thread_rng().gen_range(2..4))); }
    _ => {}
  }

  match rand::thread_rng().gen_bool(probability) {
    true => { rule = Box::new(SizeAdvantage::new(rule, rand::thread_rng().gen_range(0..11))); }
    _ => {}
  }

  return rule;
}

pub fn get_random_rule() -> Box<dyn Rule> {
  return apply_random_modifiers(get_random_base_rule());
}

pub fn generate_random_strategy() -> Strategy {
  let num_rules = rand::thread_rng().gen_range(5..10);
  let mut strategy: Strategy = Strategy::new();

  for _iter in 0..num_rules {
    strategy.add_rule(get_random_rule());
  }

  return strategy;
}
