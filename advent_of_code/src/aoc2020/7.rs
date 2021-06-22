#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::{ Regex };
use advent_of_code::get_str_array_from_file;
use itertools::Itertools;

fn get_bag(text: &str) -> (String, usize) {
  lazy_static! {
    static ref RE_BAG: Regex = Regex::new(
      r"((?P<num>\d+)\s)?(?P<color>[a-z\s]+)(?:bags?)"
    ).unwrap();
  }

  let captured = RE_BAG.captures(text).unwrap();
  let color: String = captured.name("color").unwrap().as_str().parse().unwrap();
  let num: usize = match captured.name("num") {
    Some(cap) => cap.as_str().parse().unwrap(),
    None => 1
  };
  (color, num)
}

fn get_rule(text: &str) -> (String, Vec<(String, usize)>) {
  let (master, slaves): (String, String) = text.trim_end_matches(".")
    .split(" contain ").map(|x| x.to_owned()).collect_tuple().unwrap();
  let (master_color, _) = get_bag(&master);
  let slave_bags: Vec<(String, usize)> = slaves.split(", ").map(|slave| {
    get_bag(slave)
  }).collect();
  (master_color, slave_bags)
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "7.txt"});
  let lines: Vec<(String, Vec<(String, usize)>)> = data.iter().map(|line| get_rule(&line)).collect();
  println!("{:?}", lines);
}
