use std::collections::HashMap;

use advent_of_code::get_str_array_from_file;
use itertools::Itertools;

type CaveMap = HashMap<String, Vec<String>>;

const START: &str = "start";
const END: &str = "end";

fn get_map(lines: &Vec<String>) -> CaveMap {
  let mut map: CaveMap = HashMap::new();
  lines.iter().for_each(|line| {
    let (a, b) = line.split("-").collect_tuple().unwrap();
    map.entry(a.to_string()).or_insert(vec!{}).push(b.to_string());
    map.entry(b.to_string()).or_insert(vec!{}).push(a.to_string());
  });
  map
}

fn visit(map: &CaveMap, caves: &Vec<String>, visited: &Vec<String>) -> Vec<Vec<String>> {
  caves.iter().flat_map(|cave| {
    if *cave == END {
      vec!{[visited.to_owned(), vec!{END.to_string()}].concat()}
    } else {
      let next_caves: Vec<String> = map.get(cave).unwrap().iter().filter_map(|c| {
        if c == START || (c.chars().next().unwrap().is_lowercase() && visited.contains(&c)) {
          None
        } else {
          Some(c.to_owned())
        }
      }).collect();
      visit(map, &next_caves, &([visited.to_owned(), vec!{cave.to_owned()}].concat()))
    }
  }).collect()
}

#[test]
fn test_visit() {
  let map = get_map(&vec!{
    "start-A",
    "start-b",
    "A-c",
    "A-b",
    "b-d",
    "A-end",
    "b-end",
  }.iter().map(|v| v.to_string()).collect());

  let visit_pathes = visit(&map, &vec!{START.to_string()}, &vec!{});
  assert_eq!(visit_pathes.len(), 10);

  let map = get_map(&vec!{
    "fs-end",
    "he-DX",
    "fs-he",
    "start-DX",
    "pj-DX",
    "end-zg",
    "zg-sl",
    "zg-pj",
    "pj-he",
    "RW-he",
    "fs-DX",
    "pj-RW",
    "zg-RW",
    "start-pj",
    "he-WI",
    "zg-he",
    "pj-fs",
    "start-RW",
  }.iter().map(|v| v.to_string()).collect());

  let visit_pathes = visit(&map, &vec!{START.to_string()}, &vec!{});
  assert_eq!(visit_pathes.len(), 226);
}



fn main() {
  let lines = get_str_array_from_file(&vec!{"aoc2021", "data", "12.txt"});
  let map = get_map(&lines);

  let visit_pathes = visit(&map, &vec!{START.to_string()}, &vec!{});

  println!("Part 1: {}", visit_pathes.len());
}
