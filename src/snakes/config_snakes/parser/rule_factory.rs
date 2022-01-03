use log::{
  error,
  warn
};

use crate::snakes::strategy::rules::{
  rule::Rule,
  patterns::{
    cornered::cornered,
    cutoff::cutoff,
    to_the_wall::to_the_wall
  },
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
  not_in_hazard::NotInHazard,
  go_center::GoCenter,
  min_snakes::MinSnakes,
  max_snakes::MaxSnakes,
  hazard_timer::HazardTimer
};

use crate::snakes::config_snakes::parser::strategy_parser::RuleStruct;

fn build_base_rule(rule_struct: &RuleStruct) -> Option<Box<dyn Rule>> {
  match rule_struct.name.as_ref() {
    "DontDie" => { return Some(Box::new(DontDie {})); }
    "Eat" => { return Some(Box::new(Eat {})); }
    "SpaceCalc" => { return Some(Box::new(SpaceCalc {})); }
    "HeadToHead" => { return Some(Box::new(HeadToHead {})); }
    "AvoidWalls" => { return Some(Box::new(AvoidWalls {})); }
    "AvoidHazards" => { return Some(Box::new(AvoidHazards {})); }
    "FoodSafety" => { return Some(Box::new(FoodSafety {})); }
    "ChaseOtherSnakes" => { return Some(Box::new(ChaseOtherSnakes {})); }
    "AvoidHazardWalls" => { return Some(Box::new(AvoidHazardWalls {})); }
    "TurnTowardsTail" => { return Some(Box::new(TurnTowardsTail {})); }
    "EscapeHazards" => { return Some(Box::new(EscapeHazards {})); }
    "GoCenter" => { return Some(Box::new(GoCenter {})); }

    "Cornered" => { return Some(Box::new(PatternMatch {
      pattern: cornered(),
      name: "Cornered".to_string()
    })); }

    "Cutoff" => { return Some(Box::new(PatternMatch {
      pattern: cutoff(),
      name: "Cutoff".to_string()
    })); }

    "ToTheWall" => { return Some(Box::new(PatternMatch {
      pattern: to_the_wall(),
      name: "To The Wall".to_string()
    })); }



    "RunAwayFromCloseSnakes" => {
      let parameter = &rule_struct.parameters[0];
      let parse_result = parameter.parse::<i32>();
      match parse_result {
        Ok(divisor) => {
          return Some(Box::new(RunAwayFromCloseSnakes::new(divisor)));
        }
        Err(err) => {
          error!("Failed to parse parameter {}: {}", parameter, err);
        }
      }
    }

    "RunAwayFromCloseSnakesPathDist" => {
      let parameter = &rule_struct.parameters[0];
      let parse_result = parameter.parse::<i32>();
      match parse_result {
        Ok(divisor) => {
          return Some(Box::new(RunAwayFromCloseSnakesPathDist::new(divisor)));
        }
        Err(err) => {
          error!("Failed to parse parameter {}: {}", parameter, err);
        }
      }
    }
 
    _ => {
      warn!("No rule found for name {}. Skipping this rule.", &rule_struct.name);
    }
  }

  return None;
}

pub fn build_rule(rule_struct: &RuleStruct) -> Option<Box<dyn Rule>> {
  match build_base_rule(rule_struct) {
    Some(base_rule) => {
      let mut rule: Box<dyn Rule> = base_rule;

      match rule_struct.health {
        Some(threshold) => {
          rule = Box::new(HealthThreshold::new(rule, threshold));
        }
        None => {}
      }

      match rule_struct.size_advantage {
        Some(threshold) => {
          rule = Box::new(SizeAdvantage::new(rule, threshold));
        }
        None => {}
      }

      if rule_struct.on_wall {
        rule = Box::new(OnWall::new(rule));
      }

      if rule_struct.in_hazard {
        rule = Box::new(InHazard::new(rule));
      }

      if rule_struct.not_in_hazard {
        rule = Box::new(NotInHazard::new(rule));
      }

      match rule_struct.min_snakes {
        Some(threshold) => {
          rule = Box::new(MinSnakes::new(rule, threshold));
        }
        None => {}
      }

      match rule_struct.max_snakes {
        Some(threshold) => {
          rule = Box::new(MaxSnakes::new(rule, threshold));
        }
        None => {}
      }

      if rule_struct.skippable {
        rule = Box::new(Skippable::new(rule));
      }

      if rule_struct.hazard_timer {
        rule = Box::new(HazardTimer::new(rule));
      }

      return Some(rule);
    }
    None => {}
  }

  return None;
}
