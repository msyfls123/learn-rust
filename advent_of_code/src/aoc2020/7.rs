#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::{ Regex };
use advent_of_code::get_str_array_from_file;
use itertools::Itertools;
use std::collections::HashMap;

type Rule = (String, Vec<(String, usize)>);

type BagContainerMap = HashMap<String, Vec<String>>;

fn get_bag(text: &str) -> (String, usize) {
  lazy_static! {
    static ref RE_BAG: Regex = Regex::new(
      r"((?P<num>\d+)\s)?(?P<color>[a-z\s]+[a-z])\s(?:bags?)"
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

fn get_rule(text: &str) -> Rule {
  let (master, slaves): (String, String) = text.trim_end_matches(".")
    .split(" contain ").map(|x| x.to_owned()).collect_tuple().unwrap();
  let (master_color, _) = get_bag(&master);
  let slave_bags: Vec<(String, usize)> = slaves.split(", ").map(|slave| {
    get_bag(slave)
  }).collect();
  (master_color, slave_bags)
}

fn get_container_bags(
  map: &BagContainerMap,
  color: &str
) -> Vec<String> {
  match map.get(color) {
    Some(containers) => {
      let mut children: Vec<String> = containers.iter().flat_map(|c| {
        let mut children = get_container_bags(&map, c);
        children.push(c.to_string());
        children
      }).collect();
      children.sort();
      children.dedup();
      children
    },
    None => vec!{}
  }
}

fn main() {
  let lines = get_str_array_from_file(&vec!{"aoc2020", "data", "7.txt"});
  let rules: Vec<Rule> = lines.iter().map(|line| get_rule(&line)).collect();
  let mut bag_container_map: BagContainerMap = HashMap::new();
  rules.iter().for_each(|rule| {
    let (master_color, slave_bags) = rule;
    slave_bags.iter().for_each(|(slave, _)| {
      let entry = bag_container_map.entry(slave.to_string()).or_insert(vec!{});
      (*entry).push(master_color.to_string());
    })
  });
  println!("{:?}", bag_container_map);
  let result = get_container_bags(&bag_container_map, "shiny gold");

  println!("{}", result.len());
}
