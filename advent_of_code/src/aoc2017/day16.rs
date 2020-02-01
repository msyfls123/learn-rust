extern crate regex;
#[macro_use] extern crate lazy_static;

use advent_of_code::get_str_array_from_file;
use regex::Regex;

fn spin(array: &Vec<char>, text: &str) -> Vec<char> {
  lazy_static! {
    static ref RE_SPIN: Regex = Regex::new(r"s(\d+)").unwrap();
  }
  let index = &RE_SPIN.captures(text).unwrap()[1].parse::<usize>().unwrap();
  let length = array.len();
  [&array[length - index..], &array[0..length - index]].concat()
}

fn exchange(array: &Vec<char>, text: &str) -> Vec<char> {
  lazy_static! {
    static ref RE_EXCHANGE: Regex = Regex::new(r"x(\d+)/(\d+)").unwrap();
  }
  let cap = &RE_EXCHANGE.captures(text).unwrap();
  let pos_a = cap[1].parse::<usize>().unwrap();
  let pos_b = cap[2].parse::<usize>().unwrap();
  let mut new_array = array.clone();
  new_array.swap(pos_a, pos_b);
  new_array
}

fn partner(array: &Vec<char>, text: &str) -> Vec<char> {
  lazy_static! {
    static ref RE_PARTNER: Regex = Regex::new(r"p([a-p])/([a-p])").unwrap();
  }
  let cap = &RE_PARTNER.captures(text).unwrap();
  let pos_a = array.iter().position(|&x| x == cap[1].parse::<char>().unwrap()).unwrap();
  let pos_b = array.iter().position(|&x| x == cap[2].parse::<char>().unwrap()).unwrap();
  let mut new_array = array.clone();
  new_array.swap(pos_a, pos_b);
  new_array
}

fn dance(programs: &Vec<char>, array: &Vec<String>) -> Vec<char> {
  array.iter().fold(
    programs.to_vec(),
    |acc, text| {
      match text.chars().next().unwrap() {
          's' => spin(&acc, text),
          'x' => exchange(&acc, text),
          'p' => partner(&acc, text),
          _ => acc,
      }
    }
  )
}

fn main() {
  let array: Vec<String> = get_str_array_from_file(&vec!{"aoc2017", "day16_data.txt"})
    .first()
    .unwrap()
    .split(",")
    .map(|x| x.to_string())
    .collect();
  let initial_programs = (0..16).map(|x| ((x as u8) + 97 ) as char).collect::<Vec<char>>();

  let dance_once = dance(
    &initial_programs,
    &array,
  );
  println!("Part 1: {}", dance_once.clone().into_iter().collect::<String>());

  let mut cycle_programs: Vec<Vec<char>> = vec!{initial_programs.clone()};
  let mut programs = dance_once.clone();
  while programs != initial_programs {
    cycle_programs.push(programs.clone());
    programs = dance(&programs, &array);
  }
  let dance_billion_times = &cycle_programs[1_000_000_000 % cycle_programs.len()];
  println!("Part 2: {}", dance_billion_times.clone().into_iter().collect::<String>());
}