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

/// * `is_second_life` - A boolean value means whether meet small cave before
fn visit(map: &CaveMap, caves: &Vec<String>, visited: &Vec<String>, is_second_life: bool) -> Vec<Vec<String>> {
  caves.iter().flat_map(|cave| {
    let mut is_second_life = is_second_life;
    if cave.chars().next().unwrap().is_lowercase() && visited.contains(cave) {
      if is_second_life {
        return vec!{};
      } else {
        is_second_life = true;
      }
    }

    if *cave == END {
      vec!{[visited.to_owned(), vec!{END.to_string()}].concat()}
    } else {
      let next_caves: Vec<String> = map.get(cave).unwrap().iter().filter_map(|c| {
        if c == START {
          None
        } else {
          Some(c.to_owned())
        }
      }).collect();
      visit(map, &next_caves, &([visited.to_owned(), vec!{cave.to_owned()}].concat()), is_second_life)
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

  let visit_pathes = visit(&map, &vec!{START.to_string()}, &vec!{}, true);
  let visit_pathes_2 = visit(&map, &vec!{START.to_string()}, &vec!{}, false);
  assert_eq!(visit_pathes.len(), 10);
  assert_eq!(visit_pathes_2.len(), 36);

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

  let visit_pathes = visit(&map, &vec!{START.to_string()}, &vec!{}, true);
  let visit_pathes_2 = visit(&map, &vec!{START.to_string()}, &vec!{}, false);
  assert_eq!(visit_pathes.len(), 226);
  assert_eq!(visit_pathes_2.len(), 3509);
}



fn main() {
  let lines = get_str_array_from_file(&vec!{"aoc2021", "data", "12.txt"});
  let map = get_map(&lines);

  let visit_pathes = visit(&map, &vec!{START.to_string()}, &vec!{}, true);

  println!("Part 1: {}", visit_pathes.len());

  let visit_pathes_2 = visit(&map, &vec!{START.to_string()}, &vec!{}, false);

  println!("Part 2: {}", visit_pathes_2.len());
}
