extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;
use advent_of_code::get_str_array_from_file;

fn get_acceleration(text: &str) -> i32 {
  lazy_static! {
    static ref RE_ACCELERATION: Regex = Regex::new(r"a=<([-]{0,1}\d+),([-]{0,1}\d+),([-]{0,1}\d+)>").unwrap();
  }
  let cap = &RE_ACCELERATION.captures(text).unwrap();
  let x = cap[1].parse::<i32>().unwrap().abs();
  let y = cap[2].parse::<i32>().unwrap().abs();
  let z = cap[3].parse::<i32>().unwrap().abs();
  x + y + z
}

fn get_velocity(text: &str) -> i32 {
  lazy_static! {
    static ref RE_VELOCITY: Regex = Regex::new(r"v=<([-]{0,1}\d+),([-]{0,1}\d+),([-]{0,1}\d+)>").unwrap();
  }
  let cap = &RE_VELOCITY.captures(text).unwrap();
  let x = cap[1].parse::<i32>().unwrap().abs();
  let y = cap[2].parse::<i32>().unwrap().abs();
  let z = cap[3].parse::<i32>().unwrap().abs();
  x + y + z
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2017", "day20_data.txt"});
  let v_and_a_list: Vec<(i32, i32)> = array.iter().map(|x| (get_acceleration(x), get_velocity(x))).collect();
  let min_a = v_and_a_list.iter().map(|(a, _)| a).min().unwrap();
  let min_v = v_and_a_list.iter().filter(|(a, _)| a == min_a).map(|(_, v)| v).min().unwrap();
  let min_position = v_and_a_list.iter().position(|&x| x == (*min_a, *min_v)).unwrap();
  println!("Part 1: {}", min_position);
}