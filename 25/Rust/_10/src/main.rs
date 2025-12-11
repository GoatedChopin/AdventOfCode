use std::{collections::{HashSet, VecDeque}, fs};

#[derive(Debug, Clone)]
struct Machine {
    desired: u16,
    buttons: Vec<u16>,
    button_joltages: Vec<Vec<usize>>,
    joltages: Vec<u16>,
}

#[derive(Debug, Clone)]
struct MachineState {
  current: u16,
  num_presses: u32,
}

#[derive(Debug, Clone)]
struct JoltageState {
  current: Vec<u16>,
  num_presses: u32,
}

enum ParseState {
    Desired,
    Buttons,
    Dimensions,
}

impl Machine {
    fn print(&self) {
        println!("Desired: {:08b}", self.desired);
        for button in &self.buttons {
            println!(" Button: {:08b}", button);
        }
    }
    fn from_str(s: &str) -> Self {
        let mut state = ParseState::Desired;
        let parts = s.split(" ").collect::<Vec<&str>>();
        let mut desired = 0;
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();
        let mut button_joltages = Vec::new();
        for (i, part) in parts.iter().enumerate() {
            match state {
                ParseState::Desired => {
                    let desired_unformatted = part;
                    for (i, c) in desired_unformatted.replace("[", "").replace("]", "").chars().enumerate() {
                        if c == '#' {
                            desired |= 1 << i;
                        }
                    }
                    state = ParseState::Buttons;
                }
                ParseState::Buttons => {
                    if i == parts.len() - 2 {
                        state = ParseState::Dimensions;
                    }
                    let flip_bits: Vec<u16> = part
                        .replace("(", "")
                        .replace(")", "")
                        .split(",")
                        .map(|x| x.parse().unwrap())
                        .collect();
                    button_joltages.push(flip_bits.iter().map(|x| *x as usize).collect());
                    let mut button = 0;
                    for bit in flip_bits {
                        button |= 1 << bit;
                    }
                    buttons.push(button);
                }
                ParseState::Dimensions => {
                  joltages = part
                    .replace("{", "")
                    .replace("}", "")
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect();
                }
            }
        }
        Self {
            desired,
            buttons,
            button_joltages,
            joltages,
        }
    }
}

fn read_input(path: &str) -> Vec<Machine> {
    let lines = fs::read_to_string(path).expect("Failed to read input file");
    lines.lines().map(|line| Machine::from_str(line)).collect()
}

fn minimum_presses(machine: &Machine) -> u32 {
  let presses = 0;
  let mut queue = VecDeque::new();
  queue.push_back(MachineState {
    current: 0,
    num_presses: 0,
  });
  while !queue.is_empty() {
    let m_state = queue.pop_front().unwrap();
    // println!("Current: {:08b}, want {:08b}", m_state.current, machine.desired);
    if m_state.current == machine.desired {
      return m_state.num_presses;
    }
    for button in &machine.buttons {
      let mut new_machine = m_state.clone();
      new_machine.current ^= button;
      new_machine.num_presses += 1;
      queue.push_back(new_machine);
    }
  }
  presses
}

fn part_one(machines: &Vec<Machine>) -> u32 {
  let mut all_presses = 0;
  for machine in machines {
    // machine.print();
    all_presses += minimum_presses(machine);
  }
  all_presses
}

fn minimum_joltages(machine: &Machine) -> u32 {
  let mut queue = VecDeque::new();
  queue.push_back(JoltageState {
    current: vec![0; machine.joltages.len()],
    num_presses: 0,
  });
  let mut visited = HashSet::new();
  let mut max_presses = 0;
  while !queue.is_empty() {
    let j_state = queue.pop_front().unwrap();
    if j_state.num_presses > max_presses {
      println!("New max presses: {}, checking {:?} states", j_state.num_presses, queue.len());
      max_presses = j_state.num_presses;
    }

    // println!("Current: {:?}, want {:?}", j_state.current, machine.joltages);
    if j_state.current == machine.joltages {
      return j_state.num_presses;
    }
    if machine.joltages.iter().zip(j_state.current.iter()).any(|(j, c)| j < c) {
      continue;
    }
    for button_joltage in &machine.button_joltages {
      let mut new_joltage = j_state.clone();
      for slot in button_joltage {
        new_joltage.current[*slot] += 1;
        if new_joltage.current[*slot] > machine.joltages[*slot] {
          // println!("Invalid state: {:?}, want {:?}", new_joltage.current, machine.joltages);
          continue;
        }
        if visited.contains(&new_joltage.current) {
          continue;
        }
        visited.insert(new_joltage.current.clone());
      }
      new_joltage.num_presses += 1;
      queue.push_back(new_joltage);
    }
  }
  panic!("No solution found");
}

fn part_two(machines: &Vec<Machine>) -> u32 {
  let mut all_joltages = 0;
  for machine in machines {
    machine.print();
    all_joltages += minimum_joltages(machine);
  }
  all_joltages
}

fn test() {
    let machine = Machine::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    assert_eq!(machine.desired, 0b00110);
    let input = read_input("test.txt");
    assert_eq!(part_one(&input), 7);
    assert_eq!(part_two(&input), 33);
}

fn main() {
    test();
    let input = read_input("input.txt");
    // println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
