use std::{collections::{HashMap, BinaryHeap}, cell::RefCell, rc::Rc, cmp::Ordering};

use advent_of_code::get_str_array_from_file;

type Position = (usize, usize);
type RiskLevelMap = HashMap<Position, usize>;
type RiskCacheMap = RiskLevelMap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
  cost: usize,
  position: Position,
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

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

fn get_5x_map<T>(lines: &Vec<T>) -> RiskLevelMap
where
  T: Into<String> + Clone,
{
  let length = lines.len();
  let mut map = HashMap::new();
  for (y, row) in lines.iter().enumerate() {
    let row_str: String = row.clone().into();
    for (x, c) in row_str.chars().enumerate() {
      let val = c.to_digit(10).unwrap() as usize;
      for ix in 0..5 {
        for iy in 0..5 {
          map.insert((x + ix * length, y + iy * length), (val + ix + iy - 1) % 9 + 1);
        }
      }
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

/**
 * https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples
 */
fn shortest_path(map: &RiskLevelMap, length: usize, start: Position, goal: Position) -> Option<usize> {
  let mut dist: RiskLevelMap = HashMap::new();
  map.iter().for_each(|(&pos, _)| { dist.insert(pos, usize::MAX); });

  let mut heap = BinaryHeap::new();

  dist.insert(start, 0);
  heap.push(State { cost: 0, position: start });

  while let Some(State { cost, position }) = heap.pop() {
    if position == goal { return Some(cost) };

    if &cost > dist.get(&position).unwrap() { continue; }

    let adj_list: Vec<Position> = [
      position.0.checked_sub(1).map(|x| (x, position.1)),
      if position.0 < length - 1 { Some((position.0 + 1, position.1)) } else { None },
      position.1.checked_sub(1).map(|y| (position.0, y)),
      if position.1 < length - 1 { Some((position.0, position.1 + 1)) } else { None },
    ].iter().filter_map(|&v| v).collect();

    for &next_point in &adj_list {
      let next = State { cost: cost + map.get(&next_point).unwrap(), position: next_point };

      if &next.cost < dist.get(&next_point).unwrap() {
        heap.push(next);
        dist.insert(next_point, next.cost);
      }
    }
  }

  None
}

#[test]
fn test_shortest_path() {
  let lines = vec!{
    "1163751742",
    "1381373672",
    "2136511328",
    "3694931569",
    "7463417111",
    "1319128137",
    "1359912421",
    "3125421639",
    "1293138521",
    "2311944581",
  };
  let map = get_5x_map(&lines);
  let length = lines.len() * 5;
  let lowest_risk = shortest_path(&map, length, (0, 0), (length - 1, length - 1));
  assert_eq!(lowest_risk, Some(315));
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
  let new_map = get_5x_map(&lines);
  let length = length * 5;
  let lowest_risk = shortest_path(
    &new_map,
    length,
    (0, 0),
    (length - 1, length - 1),
  );
  println!("Part 2: {}", lowest_risk.unwrap());
}
