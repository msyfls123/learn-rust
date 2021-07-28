#[macro_use] extern crate lazy_static;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;

pub mod aoc2017;

pub fn get_str_from_file(path_list: &Vec<&str>) -> String {
  let path = path_list.iter().fold(
    Path::new("advent_of_code").join("src"),
    |acc, x| acc.join(x)
  );
  let mut file = File::open(path).unwrap();
  let mut s = String::new();
  file.read_to_string(&mut s).unwrap();
  s
}

pub fn get_str_array_from_file(path_list: &Vec<&str>) -> Vec<String> {
  let s = get_str_from_file(&path_list);
  let array: Vec<String> = s.lines().filter_map(|x| {
    if x == "" {
        None
    } else {
        Some(x.to_string())
    }
  }).collect();
  array
}

pub fn get_group_str_from_file(path_list: &Vec<&str>) -> Vec<Vec<String>> {
  let data: Vec<String> = get_str_from_file(&path_list)
    .lines()
    .map(|line| line.to_string()).collect();
  data
    .into_iter()
    .group_by(|line| line == "")
    .into_iter()
    .filter_map(|(_match, group)| {
      let lines: Vec<String> = group.collect();
      if lines == vec!{String::from("")} {
        return None
      }
      Some(lines)
    })
    .collect()
}
