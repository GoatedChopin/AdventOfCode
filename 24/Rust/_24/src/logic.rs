#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Condition {
  Equal,
  NotEqual,
}

struct Constraint {
  left_register: Register,
  right_register: Register,
  right_literal_value: Option<bool>,
  condition: Condition,
}