#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::collections::HashMap;
use advent_of_code::get_str_from_file;

#[derive(Deserialize, Debug)]
struct Recipe {
  write: isize,
  step: isize,
  next: char,
}

#[derive(Deserialize, Debug)]
struct Config {
  state: char,
  zero: Recipe,
  one: Recipe,
}

#[derive(Deserialize, Debug)]
struct Blueprint {
  begin: char,
  steps: usize,
  configs: Vec<Config>,
}

type Tape = HashMap<isize, isize>;

fn run(
  tape: &mut Tape,
  index: isize,
  state: char,
  configs: &Vec<Config>
) -> (isize, char) {
  let current = tape.entry(index).or_insert(0);
  let config = configs.iter().find(|&c| c.state == state).unwrap();
  match *current {
    0 => {
      *current = config.zero.write;
      (index + config.zero.step, config.zero.next)
    },
    1 => {
      *current = config.one.write;
      (index + config.one.step, config.one.next)
    },
    _ => panic!("not found")
  }
}

fn main() {
  let json = get_str_from_file(&vec!{"aoc2017", "day25_data.json"});
  let blueprint = serde_json::from_str::<Blueprint>(&json).unwrap();
  let mut tape: Tape = HashMap::new();
  let mut state: char = blueprint.begin;
  let mut index: isize = 0;
  let mut count: usize = 0;
  let steps: usize = blueprint.steps;
  while count < steps {
    let result = run(&mut tape, index, state, &blueprint.configs);
    index = result.0;
    state = result.1;
    count += 1;
  };
  let total = tape.values().filter(|&x| *x == 1).count();
  println!("Part 1: {}", total);
}
