#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SpaceType {
  Food,
  Snake,
  Wall,
  Empty,
  Tail,
  Head,
}

impl SpaceType {
  pub fn is_navigable(&self) -> bool {
    match self {
      SpaceType::Empty |
      SpaceType::Food |
      SpaceType::Tail => { true }
      _ => { false }
    }
  }
}
