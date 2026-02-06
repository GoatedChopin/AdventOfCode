struct Register {
  name: [char; 3],
  viable_values: HashSet<bool>,
}

impl Register {
  fn new(name: [char; 3]) -> Self {
    Self {
      name,
      viable_values: HashSet::from([false, true]),
    }
  }
}