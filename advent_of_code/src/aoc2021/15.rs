use std::{collections::HashMap, cell::RefCell, rc::Rc};

use advent_of_code::get_str_array_from_file;

type Position = (usize, usize);
type RiskLevelMap = HashMap<Position, usize>;
type RiskCacheMap = RiskLevelMap;

fn get_map<T>(lines: &Vec<T>) -> RiskLevelMap
  where T: Into<String> + Clone,   {
  let mut map = HashMap::new();
  for (y, row) in lines.iter().enumerate() {
    let row_str: String = row.clone().into();
    for (x, v) in row_str.chars().enumerate() {
      map.insert((x, y), v.to_digit(10).unwrap() as usize);
    }
  }
  map
}

fn find_lowest_total_risk(
  map: &RiskLevelMap,
  cache: Rc<RefCell<RiskCacheMap>>,
  point: &Position,
) -> usize {
  if point == &(0, 0) {
    return 0;
  }

  [
    point.0.checked_sub(1).map(|x| (x, point.1)),
    point.1.checked_sub(1).map(|y| (point.0, y)),
  ].iter().filter_map(|pos| {
    if pos.is_some() {
      let pos = pos.unwrap();
      let option_risk = cache.borrow().get(&pos).map(|x| x.to_owned());
      match option_risk {
        Some(risk) => Some(risk),
        None => {
          let risk = find_lowest_total_risk(
            map, Rc::clone(&cache), &pos
          );
          cache.borrow_mut().insert(pos, risk);
          Some(risk)
        }
      }
    } else {
      None
    }
  }).min().unwrap() + map.get(point).unwrap()
}

#[test]
fn test_find_lowest_total_risk() {
  let lines = vec!{
    "116",
    "138",
    "213",
  };
  let map = get_map(&lines);
  let cache = HashMap::new();
  let lowest_risk = find_lowest_total_risk(&map, Rc::new(RefCell::new(cache)), &(2, 2));
  assert_eq!(lowest_risk, 7);
}

fn main() {
  let lines = get_str_array_from_file(&vec!{"aoc2021", "data", "15.txt"});
  let length = lines.len();
  let map = get_map(&lines);
  let lowest_risk = find_lowest_total_risk(
    &map,
    Rc::new(RefCell::new(HashMap::new())),
    &(length - 1, length - 1),
  );
  println!("Part 1: {}", lowest_risk);
}
