use std::{collections::{HashMap, HashSet}, iter::FromIterator};

use advent_of_code::get_str_from_file;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

type TimeVelocityMap = HashMap<usize, Vec<isize>>;

#[derive(Debug, PartialEq)]
struct Area {
  x_min: isize,
  x_max: isize,
  y_min: isize,
  y_max: isize,
}

fn get_area(text: &str) -> Area {
  lazy_static! {
    static ref RE_AREA: Regex = Regex::new(r"x=(?P<x_min>[-\d]+)\.\.(?P<x_max>[-\d]+),\sy=(?P<y_min>[-\d]+)\.\.(?P<y_max>[-\d]+)").unwrap();
  }
  let cap = RE_AREA.captures(text).unwrap();
  let x_min = cap.name("x_min").unwrap().as_str().parse().unwrap();
  let x_max = cap.name("x_max").unwrap().as_str().parse().unwrap();
  let y_min = cap.name("y_min").unwrap().as_str().parse().unwrap();
  let y_max = cap.name("y_max").unwrap().as_str().parse().unwrap();
  Area {
    x_max,
    x_min,
    y_max,
    y_min
  }
}

#[test]
fn test_get_area() {
  assert_eq!(get_area("target area: x=20..30, y=-10..-5"), Area {
    x_min: 20,
    x_max: 30,
    y_min: -10,
    y_max: -5
  })
}

fn get_valid_y_map(y_min: isize, y_max: isize) -> TimeVelocityMap {
  let mut map = HashMap::new();
  let y_range = (y_min..=-1 - y_min);
  y_range.for_each(|y| {
    (1..).scan(0, |dist, t| {
      let dy = y - t + 1;
      *dist = *dist + dy;
      Some((*dist, t))
    }).take_while(|&(dist, _)| {
      dist >= y_min
    }).for_each(|(dist, t)| {
      if dist >= y_min && dist <= y_max {
        map.entry(t as usize).or_insert(vec!{}).push(y);
      }
    });
  });
  map
}

#[test]
fn test_get_valid_y_map() {
  let map = get_valid_y_map(-10, -5);
  assert_eq!(map.get(&20), Some(&vec!{9}));
  assert_eq!(map.get(&2), Some(&vec!{-4, -3, -2}));
}

fn get_valid_x_map(x_min: isize, x_max: isize, max_t: usize) -> TimeVelocityMap {
  let mut map = HashMap::new();
  let min_x = (x_min as f64).sqrt() as isize - 1;
  let max_x = x_max;
  (min_x..=max_x).for_each(|x| {
    (1..).scan(0, |dist, t| {
      let dx = if x - t + 1 > 0 { x - t + 1 } else { 0 };
      *dist = *dist + dx;
      Some((*dist, t, dx))
    }).take_while(|&(dist, t, _)| {
      if dist > x_max {
        return false;
      }

      if x - t + 1 < 0 {
        return false;
      }
      true
    }).for_each(|(dist, cur_t, dx)| {
      if dist >= x_min && dist <= x_max {
        let end_t = if dx == 0 { max_t } else { cur_t as usize };
        (cur_t as usize..=end_t).for_each(|t| {
          map.entry(t as usize).or_insert(vec!{}).push(x);
        })
      }
    });
  });
  map
}

#[test]
fn test_get_valid_x_map() {
  let map = get_valid_x_map(20, 30, 20);
  assert_eq!(map.get(&20), Some(&vec!{6, 7}));
}

fn get_distinct_velocity_count(x_map: &TimeVelocityMap, y_map: &TimeVelocityMap) -> usize {
  let all_velocity_pairs: HashSet<(isize, isize)> = x_map.iter().flat_map(|(t, x_list)| {
    y_map.get(&t).map_or(vec!{}, |y_list| {
      y_list.iter().flat_map(|y| x_list.iter().map(move |x| (*x, *y))).collect::<>()
    })
  }).collect();
  all_velocity_pairs.len()
}

#[test]
fn test_get_distinct_velocity_count() {
  let y_map = get_valid_y_map(-10, -5);
  let max_t = y_map.iter().map(|(t, _)| t).max().unwrap();
  let x_map = get_valid_x_map(20, 30, *max_t);
  assert_eq!(get_distinct_velocity_count(&x_map, &y_map), 112);
}

fn main() {
  let data = get_str_from_file(&vec!{"aoc2021", "data", "17.txt"});
  let area = get_area(&data);

  let vy = - 1 - area.y_min;
  let highest_y_position = (vy + 1) * vy / 2;
  println!("Part 1: {}", highest_y_position);

  let valid_y_map = get_valid_y_map(area.y_min, area.y_max);
  let max_t = valid_y_map.iter().map(|(t, _)| t).max().unwrap();
  let valid_x_map = get_valid_x_map(area.x_min, area.x_max, *max_t);

  let distinct_velocity_count =  get_distinct_velocity_count(&valid_x_map, &valid_y_map);
  println!("Part 2: {}", distinct_velocity_count);
}